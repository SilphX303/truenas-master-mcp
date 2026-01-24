#![allow(clippy::unwrap_used)]

use truenas_master_mcp::api_client::ApiClient;
use truenas_master_mcp::config::TrueNasConfig;

// === ApiClient Construction Tests ===

#[test]
fn test_api_client_new_with_api_key() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: Some("test-api-key-12345".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    assert!(!client.base_url().is_empty());
}

#[test]
fn test_api_client_new_with_basic_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: None,
        username: Some("admin".to_string()),
        password: Some("password123".to_string()),
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    assert!(!client.base_url().is_empty());
}

#[test]
fn test_api_client_new_without_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: None,
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 60,
        version: Default::default(),
    };

    // Should still create client (auth header will be None)
    let client = ApiClient::new(&config);
    assert!(!client.base_url().is_empty());
}

#[test]
fn test_api_client_base_url_trailing_slash() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local/".to_string(),
        api_key: Some("test-api-key-12345".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    // Should remove trailing slash
    assert_eq!(client.base_url(), "https://truenas.local");
}

#[test]
fn test_api_client_clone() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: Some("test-api-key-12345".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client1 = ApiClient::new(&config);
    let client2 = client1.clone();
    assert_eq!(client1.base_url(), client2.base_url());
}

#[test]
fn test_api_client_debug_format() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: Some("secret-api-key".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    let debug_str = format!("{:?}", client);
    // Debug should work without panicking
    assert!(debug_str.contains("truenas.local"));
}

// === Auth Header Tests ===

#[test]
fn test_api_client_bearer_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: Some("my-api-key-12345".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    // The auth header should be set for Bearer auth
    let auth = client.auth_header();
    assert!(auth.is_some());
    assert!(auth.unwrap().starts_with("Bearer "));
}

#[test]
fn test_api_client_basic_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: None,
        username: Some("admin".to_string()),
        password: Some("password".to_string()),
        verify_ssl: true,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    // The auth header should be set for Basic auth
    let auth = client.auth_header();
    assert!(auth.is_some());
    assert!(auth.unwrap().starts_with("Basic "));
}

#[test]
fn test_api_client_no_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: None,
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    // No auth header should be set
    assert!(client.auth_header().is_none());
}

// === URL Construction Tests ===
// These tests verify the URL construction logic without making actual HTTP requests

#[test]
fn test_api_client_url_construction() {
    let config = TrueNasConfig {
        server_url: "https://truenas.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: Default::default(),
    };

    let client = ApiClient::new(&config);
    // Verify base URL is correctly set
    assert_eq!(client.base_url(), "https://truenas.local");
}
