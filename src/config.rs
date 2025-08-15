use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRootConfig {
    // basic service config
    pub service_name: String,
    pub environment: String, // development | staging | production

    // repo info
    pub github_owner: Option<String>,
    pub github_repo_name: Option<String>,
    pub github_commit_hash: Option<String>,

    // auth token generated on traceroot.ai
    pub token: String,

    // console export toggles
    pub enable_span_console_export: bool,
    pub enable_log_console_export: bool,

    // local mode flag
    pub local_mode: bool,

    // endpoints (allow override for self-hosting / testing)
    pub otlp_endpoint: Option<String>,      // e.g. https://collector.traceroot.ai/v1/traces
    pub log_sink_endpoint: Option<String>,  // optional future HTTP log sink
}

impl TraceRootConfig {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let cfg: Self = if path.as_ref().extension().and_then(|e| e.to_str()) == Some("toml") {
            toml::from_str(&content)?
        } else {
            serde_json::from_str(&content)?
        };
        Ok(cfg)
    }
}
