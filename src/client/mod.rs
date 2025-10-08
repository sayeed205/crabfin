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

    // System API Methods

    /// Get server information (requires authentication)
    pub async fn get_server_info(&self) -> JellyfinResult<crate::models::jellyfin::ServerInfo> {
        let url = self.build_url("/System/Info")?;
        let response = self.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get public server information (no authentication required)
    pub async fn get_public_server_info(&self, server_url: &str) -> JellyfinResult<crate::models::jellyfin::PublicServerInfo> {
        let normalized_url = Self::normalize_server_url(server_url)?;
        let url = format!("{}/System/Info/Public", normalized_url);
        
        // Create a temporary client without auth headers for public endpoint
        let response = self.http_client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;
            
        self.handle_response(response).await
    }

    /// Ping the server to test connectivity
    pub async fn ping(&self) -> JellyfinResult<()> {
        let url = self.build_url("/System/Ping")?;
        let response = self.get(&url).send().await?;
        
        // Ping endpoint returns a simple string, not JSON
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(error::ErrorUtils::extract_error_from_response(response).await)
        }
    }

    /// Normalize and validate a server URL
    pub fn normalize_server_url(url: &str) -> JellyfinResult<String> {
        let url = url.trim();
        
        if url.is_empty() {
            return Err(JellyfinError::invalid_url("Server URL cannot be empty"));
        }

        // Add protocol if missing
        let url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("http://{}", url)
        } else {
            url.to_string()
        };

        // Parse URL to validate it
        match reqwest::Url::parse(&url) {
            Ok(parsed_url) => {
                // Remove trailing slash for consistency
                let normalized = parsed_url.as_str().trim_end_matches('/');
                Ok(normalized.to_string())
            }
            Err(_) => Err(JellyfinError::invalid_url(&format!("Invalid server URL: {}", url))),
        }
    }

    /// Connect to a server and validate it
    pub async fn connect(&mut self, server_url: &str) -> JellyfinResult<crate::models::jellyfin::PublicServerInfo> {
        let normalized_url = Self::normalize_server_url(server_url)?;
        
        // Test connection by getting public server info
        let server_info = self.get_public_server_info(&normalized_url).await?;
        
        // Store the validated server URL
        self.server_url = Some(normalized_url);
        
        Ok(server_info)
    }

    // Authentication API Methods

    /// Authenticate with username and password
    pub async fn authenticate(&mut self, username: &str, password: &str) -> JellyfinResult<crate::models::jellyfin::AuthResponse> {
        if self.server_url.is_none() {
            return Err(JellyfinError::invalid_url("Server URL not set. Call connect() first."));
        }

        let auth_request = crate::models::request::AuthRequest::new(username, password);
        let url = self.build_url("/Users/AuthenticateByName")?;
        
        let response = self.post(&url)
            .json(&auth_request)
            .send()
            .await?;

        let auth_response: crate::models::jellyfin::AuthResponse = self.handle_response(response).await?;
        
        // Store the authentication token for future requests
        self.auth_token = Some(auth_response.access_token.clone());
        
        Ok(auth_response)
    }

    /// Logout and clear authentication token
    pub async fn logout(&mut self) -> JellyfinResult<()> {
        if self.auth_token.is_some() {
            // Try to call the logout endpoint if we have a token
            if let Ok(url) = self.build_url("/Sessions/Logout") {
                // Don't fail if logout request fails - just clear the token
                let _ = self.post(&url).send().await;
            }
        }
        
        // Clear the stored token
        self.auth_token = None;
        
        Ok(())
    }

    /// Refresh the authentication token (if supported by server)
    pub async fn refresh_token(&mut self) -> JellyfinResult<crate::models::jellyfin::AuthResponse> {
        if self.auth_token.is_none() {
            return Err(JellyfinError::Authentication("No token to refresh".to_string()));
        }

        let url = self.build_url("/Auth/Keys")?;
        
        let _response = self.post(&url).send().await?;
        
        // Note: Jellyfin doesn't have a traditional token refresh endpoint
        // This is a placeholder for potential future implementation
        // For now, we'll return an error indicating refresh is not supported
        Err(JellyfinError::Authentication("Token refresh not supported by Jellyfin API".to_string()))
    }

    /// Get current user information (requires authentication)
    pub async fn get_current_user(&self) -> JellyfinResult<crate::models::jellyfin::UserInfo> {
        if self.auth_token.is_none() {
            return Err(JellyfinError::Authentication("Authentication required".to_string()));
        }

        let url = self.build_url("/Users/Me")?;
        let response = self.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Validate current authentication token
    pub async fn validate_token(&self) -> JellyfinResult<bool> {
        if self.auth_token.is_none() {
            return Ok(false);
        }

        // Try to get current user info to validate token
        match self.get_current_user().await {
            Ok(_) => Ok(true),
            Err(JellyfinError::Authentication(_)) => Ok(false),
            Err(JellyfinError::Server { status: 401, .. }) => Ok(false),
            Err(JellyfinError::Server { status: 403, .. }) => Ok(false),
            Err(e) => Err(e), // Other errors should be propagated
        }
    }

    /// Get authentication token (if authenticated)
    pub fn get_auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }

    /// Set authentication token manually (for cases where token is stored externally)
    pub fn set_auth_token_manual(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    /// Clear authentication token manually
    pub fn clear_auth_token_manual(&mut self) {
        self.auth_token = None;
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

    #[test]
    fn test_server_url_normalization() {
        // Test adding protocol
        let url = JellyfinClient::normalize_server_url("localhost:8096").unwrap();
        assert_eq!(url, "http://localhost:8096");

        // Test removing trailing slash
        let url = JellyfinClient::normalize_server_url("http://localhost:8096/").unwrap();
        assert_eq!(url, "http://localhost:8096");

        // Test valid HTTPS URL
        let url = JellyfinClient::normalize_server_url("https://jellyfin.example.com").unwrap();
        assert_eq!(url, "https://jellyfin.example.com");

        // Test empty URL
        assert!(JellyfinClient::normalize_server_url("").is_err());
        assert!(JellyfinClient::normalize_server_url("   ").is_err());

        // Test invalid URL
        assert!(JellyfinClient::normalize_server_url("not-a-url").is_err());
    }

    #[test]
    fn test_authentication_token_management() {
        let mut client = JellyfinClient::new();
        
        // Initially not authenticated
        assert!(!client.is_authenticated());
        assert!(client.get_auth_token().is_none());

        // Set token manually
        client.set_auth_token_manual("test-token".to_string());
        assert!(client.is_authenticated());
        assert_eq!(client.get_auth_token(), Some("test-token"));

        // Clear token manually
        client.clear_auth_token_manual();
        assert!(!client.is_authenticated());
        assert!(client.get_auth_token().is_none());
    }

    #[test]
    fn test_auth_header_with_manual_token() {
        let mut client = JellyfinClient::new();
        client.set_auth_token_manual("manual-token".to_string());
        
        let header = client.build_auth_header();
        assert!(header.contains("Token=\"manual-token\""));
        assert!(header.contains("MediaBrowser Client=\"Crabfin\""));
    }
}