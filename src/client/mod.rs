use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub mod error;
pub use error::{JellyfinError, JellyfinResult};

/// Jellyfin API client for handling HTTP communication with Jellyfin servers
#[derive(Debug, Clone)]
pub struct JellyfinClient {
    /// HTTP client for making requests
    http_client: Client,
    /// Base URL of the Jellyfin server
    server_url: Option<String>,
    /// Authentication token for API requests
    auth_token: Option<String>,
    /// Unique device identifier
    device_id: String,
    /// Client application name
    client_name: String,
    /// Client application version
    client_version: String,
}

impl JellyfinClient {
    /// Create a new JellyfinClient instance
    pub fn new() -> Self {
        let device_id = Self::generate_device_id();
        let client_name = "Crabfin".to_string();
        let client_version = env!("CARGO_PKG_VERSION").to_string();

        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent(format!("{}/{}", client_name, client_version))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            server_url: None,
            auth_token: None,
            device_id,
            client_name,
            client_version,
        }
    }

    /// Generate a unique device ID for this client instance
    fn generate_device_id() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut hasher = DefaultHasher::new();
        
        // Hash system information to create a semi-persistent device ID
        if let Ok(hostname) = std::env::var("HOSTNAME") {
            hostname.hash(&mut hasher);
        }
        if let Ok(user) = std::env::var("USER") {
            user.hash(&mut hasher);
        }
        
        // Add timestamp component for uniqueness
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        timestamp.hash(&mut hasher);

        format!("crabfin-{:x}", hasher.finish())
    }

    /// Get the device ID for this client
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get the client name
    pub fn client_name(&self) -> &str {
        &self.client_name
    }

    /// Get the client version
    pub fn client_version(&self) -> &str {
        &self.client_version
    }

    /// Get the current server URL
    pub fn server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }

    /// Check if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    /// Set the server URL
    pub fn set_server_url(&mut self, url: String) {
        self.server_url = Some(url);
    }

    /// Set the authentication token
    pub fn set_auth_token(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    /// Clear the authentication token
    pub fn clear_auth_token(&mut self) {
        self.auth_token = None;
    }

    /// Get the HTTP client with proper headers configured
    pub(crate) fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    /// Build authorization header value
    pub(crate) fn build_auth_header(&self) -> String {
        let mut parts = vec![
            format!("MediaBrowser Client=\"{}\"", self.client_name),
            format!("Device=\"{}\"", self.device_id),
            format!("DeviceId=\"{}\"", self.device_id),
            format!("Version=\"{}\"", self.client_version),
        ];

        if let Some(token) = &self.auth_token {
            parts.push(format!("Token=\"{}\"", token));
        }

        parts.join(", ")
    }

    /// Construct a full URL for an API endpoint
    pub(crate) fn build_url(&self, endpoint: &str) -> Result<String> {
        let base_url = self.server_url
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Server URL not set"))?;

        let endpoint = endpoint.trim_start_matches('/');
        Ok(format!("{}/{}", base_url.trim_end_matches('/'), endpoint))
    }

    /// Handle HTTP response and convert to appropriate error types
    pub(crate) async fn handle_response<T>(&self, response: reqwest::Response) -> JellyfinResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        
        if status.is_success() {
            match response.json::<T>().await {
                Ok(data) => Ok(data),
                Err(err) => {
                    if err.is_decode() {
                        Err(JellyfinError::invalid_response(
                            "Failed to decode JSON response"
                        ))
                    } else {
                        Err(JellyfinError::Network(err))
                    }
                }
            }
        } else {
            Err(error::ErrorUtils::extract_error_from_response(response).await)
        }
    }

    /// Handle HTTP response for endpoints that return no content
    pub(crate) async fn handle_empty_response(&self, response: reqwest::Response) -> JellyfinResult<()> {
        let status = response.status();
        
        if status.is_success() {
            Ok(())
        } else {
            Err(error::ErrorUtils::extract_error_from_response(response).await)
        }
    }

    /// Create a GET request with proper headers
    pub(crate) fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.http_client
            .get(url)
            .header("Authorization", self.build_auth_header())
            .header("Accept", "application/json")
    }

    /// Create a POST request with proper headers
    pub(crate) fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.http_client
            .post(url)
            .header("Authorization", self.build_auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }

    /// Create a PUT request with proper headers
    pub(crate) fn put(&self, url: &str) -> reqwest::RequestBuilder {
        self.http_client
            .put(url)
            .header("Authorization", self.build_auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }

    /// Create a DELETE request with proper headers
    pub(crate) fn delete(&self, url: &str) -> reqwest::RequestBuilder {
        self.http_client
            .delete(url)
            .header("Authorization", self.build_auth_header())
            .header("Accept", "application/json")
    }
}

impl Default for JellyfinClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = JellyfinClient::new();
        assert!(!client.device_id().is_empty());
        assert_eq!(client.client_name(), "Crabfin");
        assert!(!client.is_authenticated());
        assert!(client.server_url().is_none());
    }

    #[test]
    fn test_auth_header_without_token() {
        let client = JellyfinClient::new();
        let header = client.build_auth_header();
        assert!(header.contains("MediaBrowser Client=\"Crabfin\""));
        assert!(header.contains(&format!("DeviceId=\"{}\"", client.device_id())));
        assert!(!header.contains("Token="));
    }

    #[test]
    fn test_auth_header_with_token() {
        let mut client = JellyfinClient::new();
        client.set_auth_token("test-token".to_string());
        let header = client.build_auth_header();
        assert!(header.contains("Token=\"test-token\""));
    }

    #[test]
    fn test_url_building() {
        let mut client = JellyfinClient::new();
        client.set_server_url("http://localhost:8096".to_string());
        
        let url = client.build_url("/System/Info").unwrap();
        assert_eq!(url, "http://localhost:8096/System/Info");
        
        let url = client.build_url("System/Info").unwrap();
        assert_eq!(url, "http://localhost:8096/System/Info");
    }
}