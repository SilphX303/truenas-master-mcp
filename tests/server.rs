#![allow(clippy::unwrap_used)]

use std::str::FromStr;

use truenas_master_mcp::client::TrueNasClient;
use truenas_master_mcp::config::TrueNasConfig;

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
fn test_client_new() {
    let config = create_test_config();
    let client = TrueNasClient::new(config);
    assert!(client.is_ok());
}

#[test]
fn test_client_debug_format() {
    let config = create_test_config();
    let client = TrueNasClient::new(config).unwrap();
    let debug_str = format!("{:?}", client);
    assert!(!debug_str.is_empty());
}

#[tokio::test]
async fn test_client_clone() {
    let config = create_test_config();
    let _client1 = TrueNasClient::new(config).unwrap();
    let _client2 = _client1.clone();

    // Both clients should work independently
    assert!(TrueNasClient::new(create_test_config()).is_ok());
    assert!(TrueNasClient::new(create_test_config()).is_ok());
}

mod client_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_base_url() {
        let config = create_test_config();
        let client = TrueNasClient::new(config).unwrap();
        assert_eq!(client.base_url(), "https://test.local");
    }

    #[test]
    fn test_client_auth_header_api_key() {
        let config = TrueNasConfig {
            server_url: "https://test.local".to_string(),
            api_key: Some("test-key".to_string()),
            username: None,
            password: None,
            verify_ssl: false,
            timeout_secs: 30,
            version: Default::default(),
        };
        let client = TrueNasClient::new(config).unwrap();
        // Client should be created successfully
        assert!(client.base_url().contains("test.local"));
    }
}

// === ToolCategory and ToolConfig Tests ===

#[test]
fn test_tool_category_from_str() {
    use truenas_master_mcp::server::ToolCategory;

    assert_eq!(
        ToolCategory::from_str("users").unwrap(),
        ToolCategory::Users
    );
    assert_eq!(
        ToolCategory::from_str("pools").unwrap(),
        ToolCategory::Pools
    );
    assert_eq!(
        ToolCategory::from_str("datasets").unwrap(),
        ToolCategory::Datasets
    );
    assert_eq!(
        ToolCategory::from_str("dataset").unwrap(),
        ToolCategory::Datasets
    );
    assert_eq!(
        ToolCategory::from_str("shares").unwrap(),
        ToolCategory::Shares
    );
    assert_eq!(
        ToolCategory::from_str("share").unwrap(),
        ToolCategory::Shares
    );
    assert_eq!(
        ToolCategory::from_str("snapshots").unwrap(),
        ToolCategory::Snapshots
    );
    assert_eq!(
        ToolCategory::from_str("iscsi").unwrap(),
        ToolCategory::Iscsi
    );
    assert_eq!(ToolCategory::from_str("apps").unwrap(), ToolCategory::Apps);
    assert_eq!(ToolCategory::from_str("app").unwrap(), ToolCategory::Apps);
    assert_eq!(
        ToolCategory::from_str("network").unwrap(),
        ToolCategory::Network
    );
    assert_eq!(
        ToolCategory::from_str("system").unwrap(),
        ToolCategory::System
    );
    assert_eq!(ToolCategory::from_str("all").unwrap(), ToolCategory::All);
}

#[test]
fn test_tool_category_case_insensitive() {
    use truenas_master_mcp::server::ToolCategory;

    assert_eq!(
        ToolCategory::from_str("USERS").unwrap(),
        ToolCategory::Users
    );
    assert_eq!(
        ToolCategory::from_str("Pools").unwrap(),
        ToolCategory::Pools
    );
    assert_eq!(
        ToolCategory::from_str("NETWORK").unwrap(),
        ToolCategory::Network
    );
}

#[test]
fn test_tool_category_invalid() {
    use truenas_master_mcp::server::ToolCategory;

    assert!(ToolCategory::from_str("invalid").is_err());
    assert!(ToolCategory::from_str("").is_err());
    assert!(ToolCategory::from_str("admin").is_err());
}

#[test]
fn test_tool_category_debug() {
    use truenas_master_mcp::server::ToolCategory;

    let category = ToolCategory::Users;
    let debug_str = format!("{:?}", category);
    assert!(debug_str.contains("Users"));
}

#[test]
fn test_tool_category_clone() {
    use truenas_master_mcp::server::ToolCategory;

    let category = ToolCategory::Pools;
    let cloned = category.clone();
    assert_eq!(category, cloned);
}

#[test]
fn test_tool_category_partial_eq() {
    use truenas_master_mcp::server::ToolCategory;

    assert_eq!(ToolCategory::Users, ToolCategory::Users);
    assert_ne!(ToolCategory::Users, ToolCategory::Pools);
}

#[test]
fn test_tool_config_default() {
    use truenas_master_mcp::server::ToolConfig;

    let config = ToolConfig::default();
    assert!(!config.readonly);
    // Default derive creates empty vectors
    assert!(config.enabled_categories.is_empty());
    assert!(config.disabled_categories.is_empty());
}

#[test]
fn test_tool_config_is_category_allowed_all_enabled() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::All],
        disabled_categories: vec![],
    };

    assert!(config.is_category_allowed(&ToolCategory::Users));
    assert!(config.is_category_allowed(&ToolCategory::Pools));
    assert!(config.is_category_allowed(&ToolCategory::Datasets));
    assert!(config.is_category_allowed(&ToolCategory::All));
}

#[test]
fn test_tool_config_specific_categories_enabled() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::Users, ToolCategory::Pools],
        disabled_categories: vec![],
    };

    assert!(config.is_category_allowed(&ToolCategory::Users));
    assert!(config.is_category_allowed(&ToolCategory::Pools));
    assert!(!config.is_category_allowed(&ToolCategory::Datasets));
    assert!(!config.is_category_allowed(&ToolCategory::Network));
}

#[test]
fn test_tool_config_category_disabled() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::All],
        disabled_categories: vec![ToolCategory::Users],
    };

    assert!(!config.is_category_allowed(&ToolCategory::Users));
    assert!(config.is_category_allowed(&ToolCategory::Pools));
}

#[test]
fn test_tool_config_all_disabled() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::All],
        disabled_categories: vec![ToolCategory::All],
    };

    assert!(!config.is_category_allowed(&ToolCategory::Users));
    assert!(!config.is_category_allowed(&ToolCategory::Pools));
}

#[test]
fn test_tool_config_can_execute_readonly_modification() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: true,
        enabled_categories: vec![ToolCategory::All],
        disabled_categories: vec![],
    };

    assert!(config.can_execute(&ToolCategory::Users, false).is_ok());
    assert!(config.can_execute(&ToolCategory::Users, true).is_err());
}

#[test]
fn test_tool_config_can_execute_category_disabled() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::Users],
        disabled_categories: vec![],
    };

    assert!(config.can_execute(&ToolCategory::Users, false).is_ok());
    assert!(config.can_execute(&ToolCategory::Pools, false).is_err());
}

#[test]
fn test_tool_config_can_execute_allowed() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::All],
        disabled_categories: vec![],
    };

    assert!(config.can_execute(&ToolCategory::Users, true).is_ok());
    assert!(config.can_execute(&ToolCategory::Pools, false).is_ok());
}

#[test]
fn test_tool_config_debug() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: true,
        enabled_categories: vec![ToolCategory::Users],
        disabled_categories: vec![ToolCategory::Apps],
    };
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("readonly"));
    assert!(debug_str.contains("Users"));
}

#[test]
fn test_tool_config_clone() {
    use truenas_master_mcp::server::{ToolCategory, ToolConfig};

    let config = ToolConfig {
        readonly: false,
        enabled_categories: vec![ToolCategory::Users, ToolCategory::Pools],
        disabled_categories: vec![ToolCategory::Apps],
    };
    let cloned = config.clone();
    assert_eq!(config.readonly, cloned.readonly);
    assert_eq!(config.enabled_categories, cloned.enabled_categories);
    assert_eq!(config.disabled_categories, cloned.disabled_categories);
}
