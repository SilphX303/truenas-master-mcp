use truenas_master_mcp::error::{Result, TrueNasError};

#[test]
fn test_api_error_format() {
    let err = TrueNasError::ApiError {
        status: 404,
        message: "Not found".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("404"));
    assert!(msg.contains("Not found"));
}

#[test]
fn test_api_error_format_500() {
    let err = TrueNasError::ApiError {
        status: 500,
        message: "Internal server error".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("500"));
    assert!(msg.contains("Internal server error"));
}

#[test]
fn test_config_error_format() {
    let err = TrueNasError::ConfigError("Invalid URL".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Invalid configuration"));
    assert!(msg.contains("Invalid URL"));
}

#[test]
fn test_auth_error_format() {
    let err = TrueNasError::AuthError("Invalid credentials".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Authentication failed"));
    assert!(msg.contains("Invalid credentials"));
}

#[test]
fn test_not_found_error_format() {
    let err = TrueNasError::NotFound("User not found".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Not found"));
    assert!(msg.contains("User not found"));
}

#[test]
fn test_serialization_error_format() {
    let err = TrueNasError::SerializationError(
        serde_json::from_str::<serde_json::Value>("invalid").unwrap_err(),
    );
    let msg = err.to_string();
    assert!(msg.contains("Serialization error"));
}

#[tokio::test]
async fn test_request_error_from() {
    // Create a reqwest::Error using an invalid URL (won't actually connect)
    // This is a valid way to create a reqwest::Error for testing
    let url = reqwest::Url::parse("http://invalid.invalid").unwrap();
    let err = reqwest::Client::new().get(url).send().await.unwrap_err();
    let err: TrueNasError = err.into();
    let msg = err.to_string();
    assert!(msg.contains("HTTP request failed"));
}

#[test]
fn test_io_error_from() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: TrueNasError = io_err.into();
    let msg = err.to_string();
    assert!(msg.contains("IO error"));
}

#[test]
fn test_result_type_alias() {
    // Test that Result<T> works correctly
    fn returns_result() -> Result<i32> {
        Ok(42)
    }

    fn returns_error() -> Result<i32> {
        Err(TrueNasError::NotFound("test".to_string()))
    }

    assert_eq!(returns_result().unwrap(), 42);
    assert!(returns_error().is_err());
}

#[test]
fn test_error_debug_format() {
    let err = TrueNasError::ApiError {
        status: 500,
        message: "Server error".to_string(),
    };
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("ApiError"));
    assert!(debug_str.contains("500"));
    assert!(debug_str.contains("Server error"));
}

#[test]
fn test_error_clone() {
    // Note: TrueNasError cannot implement Clone because reqwest::Error,
    // serde_json::Error, and std::io::Error don't implement Clone.
    // This test verifies that the error can be used in contexts where Clone
    // isn't needed.
    let err = TrueNasError::NotFound("test item".to_string());
    let msg = err.to_string();
    assert!(msg.contains("test item"));
    assert!(msg.contains("Not found"));
}

#[test]
fn test_error_send_sync() {
    // Ensure errors are Send + Sync
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<TrueNasError>();
    assert_sync::<TrueNasError>();
}

#[test]
fn test_partial_eq() {
    // Note: TrueNasError cannot implement PartialEq because reqwest::Error,
    // serde_json::Error, and std::io::Error don't implement PartialEq.
    // This test verifies error equality for variants that don't contain
    // non-Comparable types.
    let err1 = TrueNasError::ConfigError("error".to_string());
    let err2 = TrueNasError::ConfigError("error".to_string());
    // Can't compare directly, but we can compare their string representations
    assert_eq!(err1.to_string(), err2.to_string());
}

#[test]
fn test_various_status_codes() {
    for status in [400, 401, 403, 404, 500, 502, 503] {
        let err = TrueNasError::ApiError {
            status,
            message: format!("Error code {}", status),
        };
        let msg = err.to_string();
        assert!(msg.contains(&format!("{}", status)));
    }
}

// === New Error Type Tests ===

#[test]
fn test_validation_error_format() {
    let err = TrueNasError::ValidationError("Invalid input format".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Validation error"));
    assert!(msg.contains("Invalid input format"));
}

#[test]
fn test_permission_denied_error_format() {
    let err = TrueNasError::PermissionDenied("Cannot modify system config".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Permission denied"));
    assert!(msg.contains("Cannot modify system config"));
}

#[test]
fn test_already_exists_error_format() {
    let err = TrueNasError::AlreadyExists("User 'admin' already exists".to_string());
    let msg = err.to_string();
    assert!(msg.contains("already exists"));
    assert!(msg.contains("User 'admin'"));
}

#[test]
fn test_timeout_error_format() {
    let err = TrueNasError::TimeoutError("Pool scrub took too long".to_string());
    let msg = err.to_string();
    assert!(msg.contains("timed out"));
    assert!(msg.contains("Pool scrub took too long"));
}

#[test]
fn test_pool_error_format() {
    let err = TrueNasError::PoolError("Pool is resilvering".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Pool error"));
    assert!(msg.contains("Pool is resilvering"));
}

#[test]
fn test_dataset_error_format() {
    let err = TrueNasError::DatasetError("Quota exceeded".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Dataset error"));
    assert!(msg.contains("Quota exceeded"));
}

#[test]
fn test_vm_error_format() {
    let err = TrueNasError::VmError("VM not running".to_string());
    let msg = err.to_string();
    assert!(msg.contains("VM error"));
    assert!(msg.contains("VM not running"));
}

#[test]
fn test_service_error_format() {
    let err = TrueNasError::ServiceError("Service stopped".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Service error"));
    assert!(msg.contains("Service stopped"));
}

#[test]
fn test_system_error_format() {
    let err = TrueNasError::SystemError("Reboot required".to_string());
    let msg = err.to_string();
    assert!(msg.contains("System error"));
    assert!(msg.contains("Reboot required"));
}

#[test]
fn test_internal_error_format() {
    let err = TrueNasError::InternalError("Unexpected null value".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Internal error"));
    assert!(msg.contains("Unexpected null value"));
}

// === Helper Method Tests ===

#[test]
fn test_not_found_helper() {
    let err = TrueNasError::not_found("User", "john");
    let msg = err.to_string();
    assert!(msg.contains("User 'john' not found"));
}

#[test]
fn test_validation_helper() {
    let err = TrueNasError::validation("Field 'email' is required");
    let msg = err.to_string();
    assert!(msg.contains("Validation error"));
    assert!(msg.contains("Field 'email' is required"));
}

#[test]
fn test_permission_denied_helper() {
    let err = TrueNasError::permission_denied("delete system dataset");
    let msg = err.to_string();
    assert!(msg.contains("Permission denied"));
    assert!(msg.contains("delete system dataset"));
}

#[test]
fn test_already_exists_helper() {
    let err = TrueNasError::already_exists("Group", "wheel");
    let msg = err.to_string();
    assert!(msg.contains("Group 'wheel' already exists"));
}

#[test]
fn test_from_api_response() {
    let err = TrueNasError::from_api_response(409, "Conflict");
    let msg = err.to_string();
    assert!(msg.contains("409"));
    assert!(msg.contains("Conflict"));
}

// === Additional Helper Method Tests ===

#[test]
fn test_pool_error_helper() {
    let err = TrueNasError::pool_error("Pool is resilvering");
    let msg = err.to_string();
    assert!(msg.contains("Pool error"));
    assert!(msg.contains("resilvering"));
}

#[test]
fn test_dataset_error_helper() {
    let err = TrueNasError::dataset_error("Quota exceeded");
    let msg = err.to_string();
    assert!(msg.contains("Dataset error"));
    assert!(msg.contains("Quota exceeded"));
}

#[test]
fn test_vm_error_helper() {
    let err = TrueNasError::vm_error("VM not running");
    let msg = err.to_string();
    assert!(msg.contains("VM error"));
    assert!(msg.contains("not running"));
}

#[test]
fn test_service_error_helper() {
    let err = TrueNasError::service_error("Service stopped");
    let msg = err.to_string();
    assert!(msg.contains("Service error"));
    assert!(msg.contains("stopped"));
}

#[test]
fn test_timeout_helper() {
    let err = TrueNasError::timeout("pool scrub");
    let msg = err.to_string();
    assert!(msg.contains("timed out"));
    assert!(msg.contains("pool scrub"));
}

#[test]
fn test_auth_error_helper() {
    let err = TrueNasError::auth_error("Invalid token");
    let msg = err.to_string();
    assert!(msg.contains("Authentication failed"));
    assert!(msg.contains("Invalid token"));
}

// === Error Code Tests ===

#[tokio::test]
async fn test_error_code_request() {
    // Create a request error using an invalid URL
    let url = reqwest::Url::parse("http://invalid.invalid").unwrap();
    let client = reqwest::Client::new();
    let err = client.get(url).send().await.unwrap_err();
    let err = TrueNasError::from(err);
    assert_eq!(err.error_code(), "REQUEST_ERROR");
}

#[test]
fn test_error_code_api() {
    let err = TrueNasError::from_api_response(404, "Not found");
    assert_eq!(err.error_code(), "API_ERROR");
}

#[test]
fn test_error_code_auth() {
    let err = TrueNasError::auth_error("Invalid credentials");
    assert_eq!(err.error_code(), "AUTH_ERROR");
}

#[test]
fn test_error_code_config() {
    let err = TrueNasError::ConfigError("Invalid config".to_string());
    assert_eq!(err.error_code(), "CONFIG_ERROR");
}

#[test]
fn test_error_code_not_found() {
    let err = TrueNasError::not_found("User", "john");
    assert_eq!(err.error_code(), "NOT_FOUND");
}

#[test]
fn test_error_code_validation() {
    let err = TrueNasError::validation("Invalid input");
    assert_eq!(err.error_code(), "VALIDATION_ERROR");
}

#[test]
fn test_error_code_permission_denied() {
    let err = TrueNasError::permission_denied("delete dataset");
    assert_eq!(err.error_code(), "PERMISSION_DENIED");
}

#[test]
fn test_error_code_already_exists() {
    let err = TrueNasError::already_exists("User", "admin");
    assert_eq!(err.error_code(), "ALREADY_EXISTS");
}

#[test]
fn test_error_code_serialization() {
    let err = TrueNasError::SerializationError(
        serde_json::from_str::<serde_json::Value>("invalid").unwrap_err(),
    );
    assert_eq!(err.error_code(), "SERIALIZATION_ERROR");
}

#[test]
fn test_error_code_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: TrueNasError = io_err.into();
    assert_eq!(err.error_code(), "IO_ERROR");
}

#[test]
fn test_error_code_timeout() {
    let err = TrueNasError::timeout("operation");
    assert_eq!(err.error_code(), "TIMEOUT_ERROR");
}

#[test]
fn test_error_code_pool() {
    let err = TrueNasError::pool_error("error");
    assert_eq!(err.error_code(), "POOL_ERROR");
}

#[test]
fn test_error_code_dataset() {
    let err = TrueNasError::dataset_error("error");
    assert_eq!(err.error_code(), "DATASET_ERROR");
}

#[test]
fn test_error_code_vm() {
    let err = TrueNasError::vm_error("error");
    assert_eq!(err.error_code(), "VM_ERROR");
}

#[test]
fn test_error_code_service() {
    let err = TrueNasError::service_error("error");
    assert_eq!(err.error_code(), "SERVICE_ERROR");
}

#[test]
fn test_error_code_system() {
    let err = TrueNasError::SystemError("error".to_string());
    assert_eq!(err.error_code(), "SYSTEM_ERROR");
}

#[test]
fn test_error_code_internal() {
    let err = TrueNasError::InternalError("error".to_string());
    assert_eq!(err.error_code(), "INTERNAL_ERROR");
}

// === Is Retryable Tests ===

#[tokio::test]
async fn test_is_retryable_request_error() {
    let url = reqwest::Url::parse("http://invalid.invalid").unwrap();
    let client = reqwest::Client::new();
    let req_err = client.get(url).send().await.unwrap_err();
    let err = TrueNasError::from(req_err);
    assert!(err.is_retryable());
}

#[test]
fn test_is_retryable_timeout_error() {
    let err = TrueNasError::timeout("operation");
    assert!(err.is_retryable());
}

#[test]
fn test_is_retryable_api_error_503() {
    let err = TrueNasError::from_api_response(503, "Service unavailable");
    assert!(err.is_retryable());
}

#[test]
fn test_is_not_retryable_auth_error() {
    let err = TrueNasError::auth_error("Invalid credentials");
    assert!(!err.is_retryable());
}

#[test]
fn test_is_not_retryable_not_found() {
    let err = TrueNasError::not_found("User", "john");
    assert!(!err.is_retryable());
}

#[test]
fn test_is_not_retryable_api_error_400() {
    let err = TrueNasError::from_api_response(400, "Bad request");
    assert!(!err.is_retryable());
}

#[test]
fn test_is_not_retryable_validation_error() {
    let err = TrueNasError::validation("Invalid input");
    assert!(!err.is_retryable());
}

// === Recovery Suggestion Tests ===

#[tokio::test]
async fn test_recovery_suggestion_request_error() {
    let url = reqwest::Url::parse("http://invalid.invalid").unwrap();
    let client = reqwest::Client::new();
    let req_err = client.get(url).send().await.unwrap_err();
    let err = TrueNasError::from(req_err);
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check network connectivity");
    assert!(!suggestion.steps.is_empty());
    assert!(suggestion.related_tools.contains(&"list_pools".to_string()));
    assert!(!suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_auth_error() {
    let err = TrueNasError::auth_error("Invalid token");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Fix authentication");
    assert!(!suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_not_found() {
    let err = TrueNasError::not_found("Pool", "tank");
    let suggestion = err.recovery_suggestion().unwrap();
    assert!(suggestion.action.contains("Pool"));
    // The related_tools contains a tool name based on the full message
    // which includes "list_pool" as a substring
    assert!(
        suggestion
            .related_tools
            .iter()
            .any(|t| t.contains("list_pool"))
    );
}

#[test]
fn test_recovery_suggestion_permission_denied() {
    let err = TrueNasError::permission_denied("delete dataset");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Request elevated permissions");
    assert!(suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_already_exists() {
    let err = TrueNasError::already_exists("User", "admin");
    let suggestion = err.recovery_suggestion().unwrap();
    assert!(suggestion.action.contains("unique identifier"));
    assert!(!suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_timeout() {
    let err = TrueNasError::timeout("scrub");
    let suggestion = err.recovery_suggestion().unwrap();
    assert!(suggestion.action.contains("scrub"));
}

#[test]
fn test_recovery_suggestion_pool_error() {
    let err = TrueNasError::pool_error("Degraded pool");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check pool status");
    assert!(suggestion.related_tools.contains(&"list_pools".to_string()));
}

#[test]
fn test_recovery_suggestion_dataset_error() {
    let err = TrueNasError::dataset_error("Quota exceeded");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check dataset configuration");
}

#[test]
fn test_recovery_suggestion_vm_error() {
    let err = TrueNasError::vm_error("Out of memory");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check VM state");
    assert!(suggestion.related_tools.contains(&"list_vms".to_string()));
}

#[test]
fn test_recovery_suggestion_service_error() {
    let err = TrueNasError::service_error("SMB stopped");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check service status");
    assert!(suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_system_error() {
    let err = TrueNasError::SystemError("Reboot required".to_string());
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Check system health");
    assert!(
        suggestion
            .related_tools
            .contains(&"get_system_info".to_string())
    );
    assert!(suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_api_error_400() {
    let err = TrueNasError::from_api_response(400, "Invalid parameter");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Fix request parameters");
    // 4xx errors require admin according to the logic: *status >= 400 && *status < 500
    assert!(suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_api_error_401() {
    let err = TrueNasError::from_api_response(401, "Unauthorized");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Re-authenticate");
}

#[test]
fn test_recovery_suggestion_api_error_403() {
    let err = TrueNasError::from_api_response(403, "Forbidden");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Request permission");
    assert!(suggestion.requires_admin);
}

#[test]
fn test_recovery_suggestion_api_error_404() {
    let err = TrueNasError::from_api_response(404, "Not found");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Verify resource exists");
}

#[test]
fn test_recovery_suggestion_api_error_409() {
    let err = TrueNasError::from_api_response(409, "Conflict");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Resolve conflict");
}

#[test]
fn test_recovery_suggestion_api_error_500() {
    let err = TrueNasError::from_api_response(500, "Internal server error");
    let suggestion = err.recovery_suggestion().unwrap();
    assert_eq!(suggestion.action, "Retry later or contact support");
}

#[test]
fn test_recovery_suggestion_none_for_internal_error() {
    let err = TrueNasError::InternalError("Unknown error".to_string());
    assert!(err.recovery_suggestion().is_none());
}
