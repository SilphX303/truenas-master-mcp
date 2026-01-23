use truenas_master_mcp::error::{TrueNasError, Result};

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
    let err = TrueNasError::SerializationError(serde_json::from_str::<serde_json::Value>("invalid").unwrap_err());
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
