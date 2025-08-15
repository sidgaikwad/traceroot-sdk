use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Simple logger that forwards to `tracing::event!`, and allows
/// per-span searchable metadata (similar to the TS SDK behavior).
#[derive(Clone, Default)]
pub struct Logger {
    meta: Arc<Mutex<Option<Value>>>,
}

impl Logger {
    pub fn info<S: AsRef<str>>(&self, msg: S) {
        if let Some(meta) = self.current_meta_blocking() {
            tracing::info!(metadata = %meta, "{}", msg.as_ref());
        } else {
            tracing::info!("{}", msg.as_ref());
        }
    }
    pub fn warn<S: AsRef<str>>(&self, msg: S) {
        if let Some(meta) = self.current_meta_blocking() {
            tracing::warn!(metadata = %meta, "{}", msg.as_ref());
        } else {
            tracing::warn!("{}", msg.as_ref());
        }
    }
    pub fn error<S: AsRef<str>>(&self, msg: S) {
        if let Some(meta) = self.current_meta_blocking() {
            tracing::error!(metadata = %meta, "{}", msg.as_ref());
        } else {
            tracing::error!("{}", msg.as_ref());
        }
    }

    /// Attach metadata for the lifetime of the returned guard.
    pub async fn with_metadata(&self, meta: Value) -> LogMetaGuard {
        let mut lock = self.meta.lock().await;
        *lock = Some(meta);
        LogMetaGuard { logger: self.clone() }
    }

    fn current_meta_blocking(&self) -> Option<String> {
        // best-effort; only used in non-async emit paths
        // (you can refactor to fully-async methods if preferred)
        futures::executor::block_on(async {
            self.meta.lock().await.as_ref().map(|v| v.to_string())
        })
    }
}

#[must_use]
pub struct LogMetaGuard {
    logger: Logger,
}
impl Drop for LogMetaGuard {
    fn drop(&mut self) {
        // on drop, clear metadata
        let logger = self.logger.clone();
        let _ = futures::executor::block_on(async {
            let mut lock = logger.meta.lock().await;
            *lock = None;
        });
    }
}

pub fn get_logger() -> Logger {
    Logger::default()
}
