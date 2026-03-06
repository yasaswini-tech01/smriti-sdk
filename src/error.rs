use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmritiError {

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Server error: {0}")]
    ServerError(String),
}