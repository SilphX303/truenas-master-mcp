use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Structured error recovery suggestion for AI assistants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySuggestion {
    /// Short action title
    pub action: String,
    /// Detailed steps to recover
    pub steps: Vec<String>,
    /// Related tools that might help
    pub related_tools: Vec<String>,
    /// Whether this requires admin access
    pub requires_admin: bool,
}

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

    /// Create a pool error
    pub fn pool_error(message: &str) -> Self {
        Self::PoolError(message.to_string())
    }

    /// Create a dataset error
    pub fn dataset_error(message: &str) -> Self {
        Self::DatasetError(message.to_string())
    }

    /// Create a VM error
    pub fn vm_error(message: &str) -> Self {
        Self::VmError(message.to_string())
    }

    /// Create a service error
    pub fn service_error(message: &str) -> Self {
        Self::ServiceError(message.to_string())
    }

    /// Create a timeout error
    pub fn timeout(operation: &str) -> Self {
        Self::TimeoutError(format!("Operation timed out: {}", operation))
    }

    /// Create an authentication error
    pub fn auth_error(message: &str) -> Self {
        Self::AuthError(message.to_string())
    }

    /// Get the error code for programmatic handling
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::RequestError(_) => "REQUEST_ERROR",
            Self::ApiError { .. } => "API_ERROR",
            Self::AuthError(_) => "AUTH_ERROR",
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::PermissionDenied(_) => "PERMISSION_DENIED",
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::SerializationError(_) => "SERIALIZATION_ERROR",
            Self::IoError(_) => "IO_ERROR",
            Self::TimeoutError(_) => "TIMEOUT_ERROR",
            Self::PoolError(_) => "POOL_ERROR",
            Self::DatasetError(_) => "DATASET_ERROR",
            Self::VmError(_) => "VM_ERROR",
            Self::ServiceError(_) => "SERVICE_ERROR",
            Self::SystemError(_) => "SYSTEM_ERROR",
            Self::InternalError(_) => "INTERNAL_ERROR",
        }
    }

    /// Check if this error indicates a retryable condition
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RequestError(_) | Self::TimeoutError(_) | Self::ApiError { status: 503, .. }
        )
    }

    /// Get a structured recovery suggestion for this error
    pub fn recovery_suggestion(&self) -> Option<RecoverySuggestion> {
        match self {
            Self::RequestError(_) => Some(RecoverySuggestion {
                action: "Check network connectivity".to_string(),
                steps: vec![
                    "Verify TrueNAS server is reachable".to_string(),
                    "Check if server URL is correct".to_string(),
                    "Ensure no firewall is blocking the connection".to_string(),
                    "Try pinging the server".to_string(),
                ],
                related_tools: vec!["list_pools".to_string()],
                requires_admin: false,
            }),
            Self::AuthError(_msg) => Some(RecoverySuggestion {
                action: "Fix authentication".to_string(),
                steps: vec![
                    "Verify API key is correct".to_string(),
                    "Check if API key has expired".to_string(),
                    "Ensure username/password credentials are valid".to_string(),
                    "Confirm user has required permissions".to_string(),
                ],
                related_tools: vec!["list_users".to_string()],
                requires_admin: false,
            }),
            Self::NotFound(resource) => Some(RecoverySuggestion {
                action: format!("Find the correct {}", resource),
                steps: vec![
                    format!("List available {} to find correct identifier", resource),
                    "Check for typos in the identifier".to_string(),
                    "Verify the resource still exists".to_string(),
                ],
                related_tools: vec![format!(
                    "list_{}",
                    resource.to_lowercase().replace(" ", "_")
                )],
                requires_admin: false,
            }),
            Self::PermissionDenied(_op) => Some(RecoverySuggestion {
                action: "Request elevated permissions".to_string(),
                steps: vec![
                    "Contact administrator for required permissions".to_string(),
                    "Verify user has correct group memberships".to_string(),
                    "Check if operation requires root/admin user".to_string(),
                ],
                related_tools: vec!["list_users".to_string(), "list_groups".to_string()],
                requires_admin: true,
            }),
            Self::AlreadyExists(resource) => Some(RecoverySuggestion {
                action: format!("Use unique identifier for {}", resource),
                steps: vec![
                    "Choose a different name/identifier".to_string(),
                    "List existing resources to find available names".to_string(),
                    "Add a timestamp or unique suffix to the name".to_string(),
                ],
                related_tools: vec![format!(
                    "list_{}",
                    resource.to_lowercase().replace(" ", "_")
                )],
                requires_admin: false,
            }),
            Self::TimeoutError(op) => Some(RecoverySuggestion {
                action: format!("Retry or increase timeout for {}", op),
                steps: vec![
                    "Retry the operation".to_string(),
                    "Increase timeout value in configuration".to_string(),
                    "Check system load during operation".to_string(),
                ],
                related_tools: vec![],
                requires_admin: false,
            }),
            Self::PoolError(_msg) => Some(RecoverySuggestion {
                action: "Check pool status".to_string(),
                steps: vec![
                    "Run list_pools to check pool status".to_string(),
                    "Check for degraded vdevs".to_string(),
                    "Verify pool has sufficient space".to_string(),
                    "Consider running a pool scrub".to_string(),
                ],
                related_tools: vec![
                    "list_pools".to_string(),
                    "get_pool_status".to_string(),
                    "scrub_pool".to_string(),
                ],
                requires_admin: false,
            }),
            Self::DatasetError(_msg) => Some(RecoverySuggestion {
                action: "Check dataset configuration".to_string(),
                steps: vec![
                    "Verify dataset exists and is not readonly".to_string(),
                    "Check available space on parent pool".to_string(),
                    "Review dataset quota settings".to_string(),
                ],
                related_tools: vec!["list_datasets".to_string(), "get_dataset".to_string()],
                requires_admin: false,
            }),
            Self::VmError(_msg) => Some(RecoverySuggestion {
                action: "Check VM state".to_string(),
                steps: vec![
                    "Run list_vms to check VM status".to_string(),
                    "Start the VM if stopped".to_string(),
                    "Check VM resources (CPU, memory)".to_string(),
                    "Review VM console for error messages".to_string(),
                ],
                related_tools: vec![
                    "list_vms".to_string(),
                    "get_vm".to_string(),
                    "start_vm".to_string(),
                ],
                requires_admin: false,
            }),
            Self::ServiceError(_msg) => Some(RecoverySuggestion {
                action: "Check service status".to_string(),
                steps: vec![
                    "Run list_services to check service state".to_string(),
                    "Start the service if stopped".to_string(),
                    "Check service dependencies".to_string(),
                    "Review service logs".to_string(),
                ],
                related_tools: vec![
                    "list_services".to_string(),
                    "get_service".to_string(),
                    "start_service".to_string(),
                ],
                requires_admin: true,
            }),
            Self::SystemError(_msg) => Some(RecoverySuggestion {
                action: "Check system health".to_string(),
                steps: vec![
                    "Run get_system_info to check system status".to_string(),
                    "Check for alerts with get_alerts".to_string(),
                    "Verify system updates are not pending".to_string(),
                    "Consider rebooting if required".to_string(),
                ],
                related_tools: vec![
                    "get_system_info".to_string(),
                    "get_alerts".to_string(),
                    "reboot_system".to_string(),
                ],
                requires_admin: true,
            }),
            Self::ApiError { status, message } => {
                let (action, steps) = match status {
                    400 => (
                        "Fix request parameters".to_string(),
                        vec![
                            "Review the error message for specific issue".to_string(),
                            "Check required fields are provided".to_string(),
                            "Validate input format".to_string(),
                        ],
                    ),
                    401 => (
                        "Re-authenticate".to_string(),
                        vec![
                            "API key may have expired".to_string(),
                            "Re-generate API key if needed".to_string(),
                        ],
                    ),
                    403 => (
                        "Request permission".to_string(),
                        vec![
                            "User lacks permission for this operation".to_string(),
                            "Contact administrator".to_string(),
                        ],
                    ),
                    404 => (
                        "Verify resource exists".to_string(),
                        vec![
                            "Resource may have been deleted".to_string(),
                            "Check identifier is correct".to_string(),
                        ],
                    ),
                    409 => (
                        "Resolve conflict".to_string(),
                        vec![
                            "Resource name conflict".to_string(),
                            "Choose a different name".to_string(),
                        ],
                    ),
                    422 => (
                        "Fix validation errors".to_string(),
                        vec![
                            "Review validation message".to_string(),
                            "Check data types and formats".to_string(),
                        ],
                    ),
                    500 | 502 | 503 => (
                        "Retry later or contact support".to_string(),
                        vec![
                            "TrueNAS server error".to_string(),
                            "Retry operation in a few minutes".to_string(),
                            "Check TrueNAS server health".to_string(),
                            "Contact support if persists".to_string(),
                        ],
                    ),
                    _ => (
                        "Review error details".to_string(),
                        vec![
                            format!("API returned status {}: {}", status, message),
                            "Consult TrueNAS documentation".to_string(),
                        ],
                    ),
                };
                Some(RecoverySuggestion {
                    action,
                    steps,
                    related_tools: vec![],
                    requires_admin: *status >= 400 && *status < 500,
                })
            }
            _ => None,
        }
    }
}
