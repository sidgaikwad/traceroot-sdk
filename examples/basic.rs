// examples/basic.rs
use serde_json::json;
use traceroot_sdk::{
    config::TraceRootConfig, get_logger, init_traceroot, trace_function,
    force_flush_tracer, shutdown_tracer, force_flush_logger, shutdown_logger,
    traceroot_trace,
};

#[tracing::instrument(skip_all)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = TraceRootConfig::from_file("traceroot.config.toml")?;
    init_traceroot(&cfg)?;

    let logger = get_logger();

    // traceFunction-like usage with metadata
    let result = trace_function("greet", Some(json!({"requestId":"123"})), || async {
        logger.info("Greeting inside traced function: world");
        "Hello, world!".to_string()
    }).await;
    logger.info(format!("Greeting result: {result}"));

    // decorator-like usage
    greet_decorated("world").await;

    force_flush_tracer().await;
    shutdown_tracer().await;
    force_flush_logger().await;
    shutdown_logger().await;
    Ok(())
}

#[traceroot_trace(span_name = "greet", trace_params = true)]
async fn greet_decorated(name: &str) -> String {
    let logger = get_logger();
    logger.info(format!("Greeting inside decorated fn: {name}"));
    "Hello!".into()
}
