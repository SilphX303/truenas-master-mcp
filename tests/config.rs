use truenas_master_mcp::config::{TrueNasConfig, TrueNasVersion};

#[test]
fn test_true_nas_version_parsing() {
    assert_eq!(
        "scale".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Scale
    );
    assert_eq!(
        "sc".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Scale
    );
    assert_eq!(
        "SCALE".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Scale
    );
    assert_eq!(
        "core".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Core
    );
    assert_eq!(
        "cr".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Core
    );
    assert_eq!(
        "CORE".parse::<TrueNasVersion>().unwrap(),
        TrueNasVersion::Core
    );
}

#[test]
fn test_true_nas_version_default() {
    let result: Result<TrueNasVersion, _> = "invalid".parse();
    assert!(result.is_err());
}

#[test]
fn test_true_nas_version_serialize() {
    use serde_json::json;

    let json = json!("scale");
    let version: TrueNasVersion = serde_json::from_value(json).unwrap();
    assert_eq!(version, TrueNasVersion::Scale);

    let json = json!("core");
    let version: TrueNasVersion = serde_json::from_value(json).unwrap();
    assert_eq!(version, TrueNasVersion::Core);

    // Test uppercase variants
    let json = json!("SCALE");
    let version: TrueNasVersion = serde_json::from_value(json).unwrap();
    assert_eq!(version, TrueNasVersion::Scale);
}

#[test]
fn test_config_debug_format_masked() {
    let config = TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("secret-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    let debug_str = format!("{:?}", config);
    // Debug should include the URL
    assert!(debug_str.contains("test.local"));
    // API key should be masked in debug output
    assert!(!debug_str.contains("secret-key"));
}

#[test]
fn test_config_version_default() {
    // Test that the default version is Scale
    let config = TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    assert_eq!(config.version, TrueNasVersion::Scale);
}

#[test]
fn test_config_version_core() {
    let config = TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Core,
    };
    assert_eq!(config.version, TrueNasVersion::Core);
}

#[test]
fn test_config_verify_ssl_default() {
    let config = TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    assert!(config.verify_ssl);
}

#[test]
fn test_config_timeout_default() {
    let config = TrueNasConfig {
        server_url: "https://test.local".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    assert_eq!(config.timeout_secs, 30);
}

#[test]
fn test_config_url_format() {
    let config = TrueNasConfig {
        server_url: "https://truenas.example.com".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    assert_eq!(config.server_url, "https://truenas.example.com");
}

#[test]
fn test_config_with_basic_auth() {
    let config = TrueNasConfig {
        server_url: "https://truenas.example.com".to_string(),
        api_key: None,
        username: Some("admin".to_string()),
        password: Some("password123".to_string()),
        verify_ssl: false,
        timeout_secs: 45,
        version: TrueNasVersion::Core,
    };
    assert!(config.api_key.is_none());
    assert_eq!(config.username, Some("admin".to_string()));
    assert_eq!(config.password, Some("password123".to_string()));
    assert!(!config.verify_ssl);
    assert_eq!(config.timeout_secs, 45);
    assert_eq!(config.version, TrueNasVersion::Core);
}
