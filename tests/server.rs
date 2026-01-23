#![allow(clippy::unwrap_used)]

use truenas_master_mcp::config::TrueNasConfig;
use truenas_master_mcp::server::TrueNasServer;

fn create_test_config() -> TrueNasConfig {
    TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    }
}

#[test]
fn test_server_new() {
    let config = create_test_config();
    let server = TrueNasServer::new(config);
    assert!(server.is_ok());
}

#[test]
fn test_server_debug_format() {
    let config = create_test_config();
    let server = TrueNasServer::new(config).unwrap();
    let debug_str = format!("{:?}", server);
    assert!(!debug_str.is_empty());
}

#[tokio::test]
async fn test_server_clone() {
    let config = create_test_config();
    let _server1 = TrueNasServer::new(config).unwrap();
    let _server2 = _server1.clone();

    // Both servers should work independently
    assert!(TrueNasServer::new(create_test_config()).is_ok());
    assert!(TrueNasServer::new(create_test_config()).is_ok());
}
