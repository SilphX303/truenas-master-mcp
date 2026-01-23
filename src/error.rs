use thiserror::Error;

/// Error types for the TrueNAS MCP server
#[derive(Error, Debug)]
pub enum TrueNasError {
    /// HTTP request failed (network issues, connection timeout, etc.)
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    /// Authentication failed (invalid credentials, expired token, etc.)
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Invalid configuration (missing env vars, invalid values, etc.)
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// Resource not found (user, group, pool, dataset, etc. doesn't exist)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Validation error (invalid input, missing required fields, etc.)
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Permission denied (insufficient privileges to perform action)
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Resource already exists (duplicate name, ID collision, etc.)
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// IO error (file system, device, etc.)
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Timeout error (operation took too long)
    #[error("Operation timed out: {0}")]
    TimeoutError(String),

    /// Pool-related errors (scrub in progress, vdev issues, etc.)
    #[error("Pool error: {0}")]
    PoolError(String),

    /// Dataset-related errors (quota exceeded, readonly dataset, etc.)
    #[error("Dataset error: {0}")]
    DatasetError(String),

    /// VM-related errors (VM not running, insufficient resources, etc.)
    #[error("VM error: {0}")]
    VmError(String),

    /// Service-related errors (service not running, dependency issues, etc.)
    #[error("Service error: {0}")]
    ServiceError(String),

    /// System-level errors (reboot required, license expired, etc.)
    #[error("System error: {0}")]
    SystemError(String),

    /// Unknown/internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, TrueNasError>;

impl TrueNasError {
    /// Create an API error from a status code and message
    pub fn from_api_response(status: u16, message: &str) -> Self {
        Self::ApiError {
            status,
            message: message.to_string(),
        }
    }

    /// Create a not found error with resource type and identifier
    pub fn not_found(resource: &str, identifier: &str) -> Self {
        Self::NotFound(format!("{} '{}' not found", resource, identifier))
    }

    /// Create a validation error
    pub fn validation(message: &str) -> Self {
        Self::ValidationError(message.to_string())
    }

    /// Create a permission denied error
    pub fn permission_denied(operation: &str) -> Self {
        Self::PermissionDenied(format!("Permission denied: {}", operation))
    }

    /// Create an already exists error
    pub fn already_exists(resource: &str, identifier: &str) -> Self {
        Self::AlreadyExists(format!("{} '{}' already exists", resource, identifier))
    }
}
