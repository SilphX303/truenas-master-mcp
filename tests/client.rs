#![allow(clippy::unwrap_used)]

use truenas_master_mcp::client::TrueNasClient;
use truenas_master_mcp::config::TrueNasConfig;

fn set_test_config(
    url: &str,
    api_key: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
) -> TrueNasConfig {
    TrueNasConfig {
        server_url: url.to_string(),
        api_key: api_key.map(|s| s.to_string()),
        username: username.map(|s| s.to_string()),
        password: password.map(|s| s.to_string()),
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    }
}

#[tokio::test]
async fn test_client_new_with_api_key() {
    let config = set_test_config("https://test.local", Some("test-api-key"), None, None);
    let client = TrueNasClient::new(config);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_client_new_with_basic_auth() {
    let config = set_test_config("https://test.local", None, Some("admin"), Some("password"));
    let client = TrueNasClient::new(config);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_client_new_without_auth() {
    let config = set_test_config("https://test.local", None, None, None);
    let client = TrueNasClient::new(config);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_client_get_success() {
    // This test would need mockito or a real server
    // For now, we just verify the client can be created
    let config = set_test_config("https://test.local", Some("key"), None, None);
    let client = TrueNasClient::new(config).unwrap();
    // The client is created successfully - integration tests would test actual HTTP
    assert!(client.base_url().starts_with("https://test.local"));
}

#[tokio::test]
async fn test_client_base_url_format() {
    let config = set_test_config("https://test.local", Some("key"), None, None);
    let client = TrueNasClient::new(config).unwrap();
    assert_eq!(client.base_url(), "https://test.local");
}

#[tokio::test]
async fn test_client_base_url_trailing_slash() {
    let config = set_test_config("https://test.local/", Some("key"), None, None);
    let client = TrueNasClient::new(config).unwrap();
    assert_eq!(client.base_url(), "https://test.local");
}

#[tokio::test]
async fn test_client_clone() {
    let config = set_test_config("https://test.local", Some("key"), None, None);
    let client1 = TrueNasClient::new(config).unwrap();
    let client2 = client1.clone();
    assert_eq!(client1.base_url(), client2.base_url());
}

#[tokio::test]
async fn test_client_debug_format() {
    let config = set_test_config("https://test.local", Some("secret-key"), None, None);
    let client = TrueNasClient::new(config).unwrap();
    let debug_str = format!("{:?}", client);
    // Debug should work without panicking
    assert!(debug_str.contains("test.local"));
}
