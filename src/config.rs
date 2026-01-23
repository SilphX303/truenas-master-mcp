use crate::error::{Result, TrueNasError};
use serde::Deserialize;
use std::env;

/// Configuration for the TrueNAS MCP server
#[derive(Debug, Clone, Deserialize)]
pub struct TrueNasConfig {
    /// TrueNAS server URL (e.g., https://truenas.local)
    pub server_url: String,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Username for basic auth (alternative to api_key)
    pub username: Option<String>,
    /// Password for basic auth (alternative to api_key)
    pub password: Option<String>,
    /// Whether to verify SSL certificates
    pub verify_ssl: bool,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for TrueNasConfig {
    fn default() -> Self {
        Self {
            server_url: env::var("TRUENAS_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost".to_string()),
            api_key: env::var("TRUENAS_API_KEY").ok(),
            username: env::var("TRUENAS_USERNAME").ok(),
            password: env::var("TRUENAS_PASSWORD").ok(),
            verify_ssl: env::var("TRUENAS_VERIFY_SSL")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            timeout_secs: env::var("TRUENAS_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}

impl TrueNasConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    fn validate(&self) -> Result<()> {
        if self.server_url.is_empty() {
            return Err(TrueNasError::ConfigError(
                "TRUENAS_SERVER_URL must be set".to_string(),
            ));
        }

        // Check if we have either API key or username/password
        if self.api_key.is_none() && (self.username.is_none() || self.password.is_none()) {
            return Err(TrueNasError::ConfigError(
                "Either TRUENAS_API_KEY or TRUENAS_USERNAME and TRUENAS_PASSWORD must be set".to_string(),
            ));
        }

        Ok(())
    }
}
