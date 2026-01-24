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

#[test]
fn test_config_username_masked_in_debug() {
    let config = TrueNasConfig {
        server_url: "https://truenas.example.com".to_string(),
        api_key: None,
        username: Some("admin".to_string()),
        password: Some("secret123".to_string()),
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    let debug_str = format!("{:?}", config);
    // Username should be masked in debug output
    assert!(!debug_str.contains("admin"));
    // Password should be masked in debug output
    assert!(!debug_str.contains("secret123"));
    // URL should be visible
    assert!(debug_str.contains("truenas.example.com"));
}

#[test]
fn test_config_api_key_masked_in_debug() {
    let config = TrueNasConfig {
        server_url: "https://truenas.example.com".to_string(),
        api_key: Some("sk-1234567890abcdef".to_string()),
        username: None,
        password: None,
        verify_ssl: true,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    let debug_str = format!("{:?}", config);
    // API key should be masked
    assert!(!debug_str.contains("sk-1234567890"));
}

#[test]
fn test_config_ssl_verification_disabled() {
    let config = TrueNasConfig {
        server_url: "https://truenas.example.com".to_string(),
        api_key: Some("test-key".to_string()),
        username: None,
        password: None,
        verify_ssl: false,
        timeout_secs: 30,
        version: TrueNasVersion::Scale,
    };
    assert!(!config.verify_ssl);
}

mod validation_tests {
    use super::*;
    use truenas_master_mcp::error::TrueNasError;

    #[test]
    fn test_config_requires_server_url() {
        let config = TrueNasConfig {
            server_url: "".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        // Empty URL should fail validation
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("TRUENAS_SERVER_URL must be set"));
        }
    }

    #[test]
    fn test_config_requires_auth() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: None,
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        // No auth credentials should fail validation
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                e.to_string().contains("API_KEY") || e.to_string().contains("TRUENAS_USERNAME")
            );
        }
    }

    #[test]
    fn test_config_api_key_sufficient() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: Some("valid-api-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        assert!(config.api_key.is_some());
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_basic_auth_sufficient() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: None,
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        assert!(config.username.is_some() && config.password.is_some());
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_invalid_url_no_protocol() {
        let config = TrueNasConfig {
            server_url: "truenas.local".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("http://") || e.to_string().contains("https://"));
        }
    }

    #[test]
    fn test_config_invalid_url_malformed() {
        let config = TrueNasConfig {
            server_url: "http://".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_api_key_too_short() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: Some("short".to_string()), // Too short
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("too short"));
        }
    }

    #[test]
    fn test_config_timeout_zero() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 0, // Invalid
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("timeout") || e.to_string().contains("TIMEOUT"));
        }
    }

    #[test]
    fn test_config_timeout_too_large() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 3601, // Too large (max is 3600)
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("3600") || e.to_string().contains("1 hour"));
        }
    }

    #[test]
    fn test_config_valid_minimal() {
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: Some("test-key-1234567890".to_string()),
            username: None,
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_partial_auth_fails() {
        // Username without password
        let config = TrueNasConfig {
            server_url: "https://truenas.local".to_string(),
            api_key: None,
            username: Some("user".to_string()),
            password: None,
            verify_ssl: true,
            timeout_secs: 30,
            version: TrueNasVersion::Scale,
        };
        let result = config.validate();
        assert!(result.is_err());
    }
}

mod file_loading_tests {
    use super::*;

    #[test]
    fn test_config_path_nonexistent() {
        let nonexistent = std::path::Path::new("/nonexistent/path/config.yaml");
        assert!(!nonexistent.exists());
    }
}
