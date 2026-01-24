#![allow(clippy::unwrap_used)]

use mockito::{Matcher, Server};
use serde_json::json;
use tokio::test as async_test;
use truenas_master_mcp::client::TrueNasClient;
use truenas_master_mcp::config::TrueNasConfig;

fn create_test_config(server_url: &str) -> TrueNasConfig {
    TrueNasConfig {
        server_url: server_url.to_string(),
        api_key: Some("test-api-key".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    }
}

#[async_test]
async fn test_get_pools() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/pool")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"[{"name": "tank", "guid": "123456", "status": "ONLINE", "size": 10000000000, "free": 5000000000}]"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let pools = client
        .get::<serde_json::Value>("/api/v2.0/pool")
        .await
        .unwrap();
    assert!(pools.is_array());
    assert_eq!(pools.as_array().unwrap().len(), 1);

    mock.assert_async().await;
}

#[async_test]
async fn test_get_users() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/user")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"[{"id": 1000, "username": "testuser", "uid": 1000}]"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let users = client
        .get::<serde_json::Value>("/api/v2.0/user")
        .await
        .unwrap();
    assert!(users.is_array());
    assert_eq!(users.as_array().unwrap()[0]["username"], "testuser");

    mock.assert_async().await;
}

#[async_test]
async fn test_get_alerts() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/system/alert")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"[{"id": "alert-1", "level": "WARNING", "message": "Pool at 80%"}]"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let alerts = client
        .get::<serde_json::Value>("/api/v2.0/system/alert")
        .await
        .unwrap();
    assert!(alerts.is_array());
    assert_eq!(alerts.as_array().unwrap()[0]["level"], "WARNING");

    mock.assert_async().await;
}

#[async_test]
async fn test_create_dataset() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/api/v2.0/pool/dataset")
        .match_header("Authorization", "Bearer test-api-key")
        .match_body(Matcher::Regex(r#".*tank.*newdata.*"#.to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"name": "tank/newdata", "pool": "tank"}"#)
        .create_async()
        .await;

    #[derive(serde::Serialize)]
    struct CreateDatasetRequest {
        name: String,
        pool: String,
    }

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let result: serde_json::Value = client
        .post(
            "/api/v2.0/pool/dataset",
            &CreateDatasetRequest {
                name: "tank/newdata".to_string(),
                pool: "tank".to_string(),
            },
        )
        .await
        .unwrap();

    assert_eq!(result["name"], "tank/newdata");

    mock.assert_async().await;
}

#[async_test]
async fn test_delete_snapshot() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("DELETE", "/api/v2.0/pool/snapshot/tank/data@snap1")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"status": "deleted"}"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let result: serde_json::Value = client
        .delete("/api/v2.0/pool/snapshot/tank/data@snap1")
        .await
        .unwrap();

    assert_eq!(result["status"], "deleted");

    mock.assert_async().await;
}

#[async_test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/pool")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(404)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"error": "Pool not found"}"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let result = client.get::<serde_json::Value>("/api/v2.0/pool").await;

    assert!(result.is_err());

    mock.assert_async().await;
}

#[async_test]
async fn test_basic_auth() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/system/info")
        .match_header("Authorization", Matcher::Regex(r#"Basic .*"#.to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"version": "TrueNAS-SCALE-24.10.0", "hostname": "test"}"#)
        .create_async()
        .await;

    let config = TrueNasConfig {
        server_url: server.url(),
        api_key: None,
        username: Some("admin".to_string()),
        password: Some("password123".to_string()),
        verify_ssl: false,
        timeout_secs: 30,
        version: Default::default(),
    };
    let client = TrueNasClient::new(config).unwrap();

    let result = client
        .get::<serde_json::Value>("/api/v2.0/system/info")
        .await
        .unwrap();
    assert_eq!(result["version"], "TrueNAS-SCALE-24.10.0");

    mock.assert_async().await;
}

#[async_test]
async fn test_get_system_info() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/system/info")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            json!({
                "version": "TrueNAS-SCALE-24.10.0",
                "hostname": "truenas.local",
                "cpu_model": "Intel Xeon E5-2670 v2",
                "uptime_seconds": 86400
            })
            .to_string(),
        )
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let info = client
        .get::<serde_json::Value>("/api/v2.0/system/info")
        .await
        .unwrap();
    assert_eq!(info["version"], "TrueNAS-SCALE-24.10.0");
    assert_eq!(info["hostname"], "truenas.local");
    assert_eq!(info["cpu_model"], "Intel Xeon E5-2670 v2");

    mock.assert_async().await;
}

#[async_test]
async fn test_list_apps() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/app")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            json!([
                {"id": "uuid-1", "name": "plex", "state": "RUNNING", "version": "1.40.0"},
                {"id": "uuid-2", "name": "nextcloud", "state": "STOPPED", "version": "28.0.0"}
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let apps = client
        .get::<serde_json::Value>("/api/v2.0/app")
        .await
        .unwrap();
    let apps_array = apps.as_array().unwrap();
    assert_eq!(apps_array.len(), 2);
    assert_eq!(apps_array[0]["name"], "plex");
    assert_eq!(apps_array[0]["state"], "RUNNING");
    assert_eq!(apps_array[1]["name"], "nextcloud");
    assert_eq!(apps_array[1]["state"], "STOPPED");

    mock.assert_async().await;
}

#[async_test]
async fn test_list_vms() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/vm")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            r#"[
            {"id": 1, "name": "windows-vm", "vcpus": 4, "memory": 8589934592, "status": "RUNNING"},
            {"id": 2, "name": "linux-vm", "vcpus": 2, "memory": 4294967296, "status": "STOPPED"}
        ]"#,
        )
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let vms = client
        .get::<serde_json::Value>("/api/v2.0/vm")
        .await
        .unwrap();
    let vms_array = vms.as_array().unwrap();
    assert_eq!(vms_array.len(), 2);
    assert_eq!(vms_array[0]["name"], "windows-vm");
    assert_eq!(vms_array[0]["vcpus"], 4);
    assert_eq!(vms_array[1]["name"], "linux-vm");

    mock.assert_async().await;
}

#[async_test]
async fn test_get_disks() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/v2.0/disk")
        .match_header("Authorization", "Bearer test-api-key")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"[
            {"devname": "ada0", "size": 4000787030016, "model": "Samsung SSD 860", "smart_status": "PASSED"},
            {"devname": "ada1", "size": 4000787030016, "model": "Samsung SSD 860", "smart_status": "PASSED"}
        ]"#)
        .create_async()
        .await;

    let config = create_test_config(&server.url());
    let client = TrueNasClient::new(config).unwrap();

    let disks = client
        .get::<serde_json::Value>("/api/v2.0/disk")
        .await
        .unwrap();
    let disks_array = disks.as_array().unwrap();
    assert_eq!(disks_array.len(), 2);
    assert_eq!(disks_array[0]["devname"], "ada0");
    assert_eq!(disks_array[0]["smart_status"], "PASSED");

    mock.assert_async().await;
}
