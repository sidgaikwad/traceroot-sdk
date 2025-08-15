pub mod config;
pub mod logger;
pub mod tracing_init;

pub use logger::{get_logger, Logger, LogMetaGuard};
pub use tracing_init::{
    force_flush_logger, force_flush_tracer, init_traceroot, shutdown_logger, shutdown_tracer,
};

/// A convenience macro that mirrors the TS `@trace` decorator.
/// Usage: `#[traceroot_trace(span_name = "greet", trace_params = true)]`
pub use traceroot_macros::traceroot_trace;

/// Helper that mirrors `traceFunction` from TS.
/// Wraps an async closure, starting a span with optional metadata.
pub async fn trace_function<F, Fut, T>(
    span_name: &str,
    mut with_meta: Option<serde_json::Value>,
    f: F,
) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let span = tracing::info_span!("traceroot", span_name = %span_name);
    let _enter = span.enter();
    if let Some(meta) = with_meta.take() {
        // attach metadata to span (searchable in UI)
        span.record("metadata", &tracing::field::display(meta));
    }
    f().await
}
