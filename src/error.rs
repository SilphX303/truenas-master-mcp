use thiserror::Error;

/// Error types for the TrueNAS MCP server
#[derive(Error, Debug)]
pub enum TrueNasError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    #[allow(dead_code)]
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TrueNasError>;
