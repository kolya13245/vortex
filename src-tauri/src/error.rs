use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum VortexError {
    #[error("Core start failed: {0}")]
    CoreStartFailed(String),

    #[error("Core not running")]
    CoreNotRunning,

    #[error("Config generation failed: {0}")]
    ConfigGenerationFailed(String),

    #[error("Subscription fetch failed: {0}")]
    SubscriptionFetchFailed(String),

    #[error("Subscription parse failed: {0}")]
    SubscriptionParseFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("{0}")]
    Other(String),
}

impl Serialize for VortexError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
