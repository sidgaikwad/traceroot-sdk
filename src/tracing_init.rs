use crate::config::TraceRootConfig;
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime::Tokio, trace as sdktrace};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static OTLP_DEFAULT: &str = "https://collector.traceroot.ai/v1/traces";

pub fn init_traceroot(cfg: &TraceRootConfig) -> anyhow::Result<()> {
    // OTLP exporter (HTTP/proto via reqwest)
    let endpoint = cfg.otlp_endpoint.clone().unwrap_or_else(|| OTLP_DEFAULT.to_string());
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            sdktrace::config()
                .with_resource(opentelemetry_sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", cfg.service_name.clone()),
                    opentelemetry::KeyValue::new("deployment.environment", cfg.environment.clone()),
                    opentelemetry::KeyValue::new("github.owner", cfg.github_owner.clone().unwrap_or_default()),
                    opentelemetry::KeyValue::new("github.repo", cfg.github_repo_name.clone().unwrap_or_default()),
                    opentelemetry::KeyValue::new("github.commit", cfg.github_commit_hash.clone().unwrap_or_default()),
                ])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint(endpoint)
                .with_headers([
                    ("authorization".into(), format!("Bearer {}", cfg.token)),
                ]),
        )
        .install_batch(Tokio)?;

    // global text map propagator
    global::set_text_map_propagator(TraceContextPropagator::new());

    // tracing subscriber
    let mut layers: Vec<Box<dyn tracing_subscriber::Layer<_> + Send + Sync>> = Vec::new();

    // span formatting to console (dev-friendly)
    if cfg.enable_span_console_export {
        layers.push(Box::new(fmt::layer().pretty()));
    }

    // JSON logs to stdout if enabled
    if cfg.enable_log_console_export {
        layers.push(Box::new(fmt::layer().json()));
    }

    // Compose subscriber
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let registry = tracing_subscriber::registry().with(filter);
    let registry = layers.into_iter().fold(registry, |reg, layer| reg.with(layer));

    registry.init();

    // keep provider alive
    let _ = tracer_provider;

    Ok(())
}

pub async fn force_flush_tracer() {
    let _ = global::force_flush_tracer_provider();
}

pub async fn shutdown_tracer() {
    let _ = global::shutdown_tracer_provider();
}

pub async fn force_flush_logger() {
    // logs go through tracing subscriber; flushing stdout/stderr is usually enough.
    use tokio::io::AsyncWriteExt;
    let _ = tokio::io::stdout().flush().await;
    let _ = tokio::io::stderr().flush().await;
}

pub async fn shutdown_logger() {
    // no-op for now; reserved for future sinks.
}
