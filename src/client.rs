use crate::error::{Result, TrueNasError};
use crate::config::TrueNasConfig;
use reqwest::{Client, header};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// TrueNAS API client
#[derive(Debug, Clone)]
pub struct TrueNasClient {
    client: Client,
    config: TrueNasConfig,
    base_url: String,
}

impl TrueNasClient {
    /// Create a new TrueNAS client
    pub fn new(config: TrueNasConfig) -> Result<Self> {
        let client = if config.verify_ssl {
            Client::builder()
                .timeout(Duration::from_secs(config.timeout_secs))
                .build()?
        } else {
            // For development/testing - disable certificate verification
            // Note: This requires the "dangerous" feature or custom TLS configuration
            // For now, we'll just use default configuration
            Client::builder()
                .timeout(Duration::from_secs(config.timeout_secs))
                .build()
                .map_err(|e| TrueNasError::ConfigError(format!(
                    "Failed to build HTTP client. For SSL verification disabled, you may need to configure certificates properly: {}", e
                )))?
        };

        let base_url = config.server_url.trim_end_matches('/').to_string();

        Ok(Self { client, config, base_url })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the authorization header value
    fn get_auth_header(&self) -> Option<String> {
        if let Some(api_key) = &self.config.api_key {
            Some(format!("Bearer {}", api_key))
        } else if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            Some(format!("Basic {}", base64_encode(username, password)))
        } else {
            None
        }
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.get(&url);

        if let Some(auth) = self.get_auth_header() {
            request = request.header(header::AUTHORIZATION, auth);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrueNasError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, endpoint: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.post(&url).json(body);

        if let Some(auth) = self.get_auth_header() {
            request = request.header(header::AUTHORIZATION, auth);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrueNasError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }

    /// Make a PUT request
    #[allow(dead_code)]
    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, endpoint: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.put(&url).json(body);

        if let Some(auth) = self.get_auth_header() {
            request = request.header(header::AUTHORIZATION, auth);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrueNasError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }

    /// Make a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.delete(&url);

        if let Some(auth) = self.get_auth_header() {
            request = request.header(header::AUTHORIZATION, auth);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrueNasError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }

    /// Make a DELETE request with body
    #[allow(dead_code)]
    pub async fn delete_with_body<T: DeserializeOwned, B: Serialize>(&self, endpoint: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.delete(&url).json(body);

        if let Some(auth) = self.get_auth_header() {
            request = request.header(header::AUTHORIZATION, auth);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrueNasError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }
}

/// Helper to create basic auth string
fn base64_encode(username: &str, password: &str) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(format!("{}:{}", username, password))
}
