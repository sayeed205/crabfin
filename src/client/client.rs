//! Jellyfin API client implementation
//!
//! This module contains the main JellyfinClient struct that handles HTTP communication
//! with Jellyfin servers, including authentication, request/response handling, and
//! connection management.

use anyhow::Result;
use reqwest::{Client, ClientBuilder, Method, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info, warn};

use super::error::{utils as error_utils, JellyfinError};

/// Main Jellyfin API client
///
/// This client handles all HTTP communication with Jellyfin servers,
/// including authentication, request building, and response parsing.
#[derive(Debug, Clone)]
pub struct JellyfinClient {
    /// The underlying HTTP client
    http_client: Client,
    /// Current server URL (None if not connected)
    server_url: Option<String>,
    /// Authentication token (None if not authenticated)
    auth_token: Option<String>,
    /// Unique device identifier
    device_id: String,
    /// Client name for identification
    client_name: String,
    /// Client version for identification
    client_version: String,
}

impl JellyfinClient {
    /// Create a new JellyfinClient instance
    ///
    /// This creates a client with default configuration but no server connection.
    /// Use `connect()` to establish a connection to a Jellyfin server.
    pub fn new() -> Self {
        let device_id = Self::generate_device_id();
        let client_name = "Crabfin".to_string();
        let client_version = env!("CARGO_PKG_VERSION").to_string();

        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent(format!("{}/{}", client_name, client_version))
            .build()
            .expect("Failed to create HTTP client");

        debug!("Created new JellyfinClient with device ID: {}", device_id);

        Self {
            http_client,
            server_url: None,
            auth_token: None,
            device_id,
            client_name,
            client_version,
        }
    }

    /// Create a new JellyfinClient with custom configuration
    pub fn with_config(
        client_name: String,
        client_version: String,
        timeout: Duration,
    ) -> Result<Self> {
        let device_id = Self::generate_device_id();

        let http_client = ClientBuilder::new()
            .timeout(timeout)
            .user_agent(format!("{}/{}", client_name, client_version))
            .build()
            .map_err(JellyfinError::Network)?;

        debug!("Created new JellyfinClient with custom config - device ID: {}", device_id);

        Ok(Self {
            http_client,
            server_url: None,
            auth_token: None,
            device_id,
            client_name,
            client_version,
        })
    }

    /// Generate a unique device ID
    ///
    /// This creates a CUID2 that uniquely identifies this client instance
    /// to the Jellyfin server.
    fn generate_device_id() -> String {
        cuid2::create_id()
    }

    /// Get the current server URL
    pub fn server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }

    /// Get the current authentication token
    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }

    /// Get the device ID
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

    /// Check if the client is connected to a server
    pub fn is_connected(&self) -> bool {
        self.server_url.is_some()
    }

    /// Check if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    /// Set the server URL
    ///
    /// This normalizes the URL by removing trailing slashes and ensuring
    /// it starts with http:// or https://
    pub fn set_server_url(&mut self, url: &str) -> Result<()> {
        let normalized_url = self.normalize_server_url(url)?;
        self.server_url = Some(normalized_url);
        debug!("Set server URL to: {:?}", self.server_url);
        Ok(())
    }

    /// Set the authentication token
    pub fn set_auth_token(&mut self, token: String) {
        self.auth_token = Some(token);
        debug!("Authentication token set");
    }

    /// Clear the authentication token
    pub fn clear_auth_token(&mut self) {
        self.auth_token = None;
        debug!("Authentication token cleared");
    }

    /// Normalize a server URL
    ///
    /// Ensures the URL has a proper scheme and removes trailing slashes
    fn normalize_server_url(&self, url: &str) -> Result<String> {
        let url = url.trim();

        if url.is_empty() {
            return Err(JellyfinError::InvalidUrl("URL cannot be empty".to_string()).into());
        }

        let normalized = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("http://{}", url)
        } else {
            url.to_string()
        };

        // Remove trailing slash
        let normalized = normalized.trim_end_matches('/').to_string();

        // Basic URL validation
        if let Err(e) = reqwest::Url::parse(&normalized) {
            return Err(JellyfinError::InvalidUrl(format!("Invalid URL format: {}", e)).into());
        }

        Ok(normalized)
    }

    /// Build a full API URL from a path
    ///
    /// Combines the server URL with the API path to create a complete URL
    fn build_api_url(&self, path: &str) -> Result<String> {
        let server_url = self.server_url
            .as_ref()
            .ok_or_else(|| JellyfinError::InvalidUrl("No server URL set".to_string()))?;

        let path = path.trim_start_matches('/');
        Ok(format!("{}/{}", server_url, path))
    }

    /// Create request headers with authentication and client identification
    fn create_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        // Add client identification headers
        if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.device_id) {
            headers.insert("X-Emby-Device-Id", value);
        }

        if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.client_name) {
            headers.insert("X-Emby-Device-Name", value);
        }

        if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.client_version) {
            headers.insert("X-Emby-Client-Version", value);
        }

        // Add authentication header if token is available
        if let Some(token) = &self.auth_token {
            let auth_value = format!("MediaBrowser Client=\"{}\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\", Token=\"{}\"",
                                     self.client_name, self.client_name, self.device_id, self.client_version, token);

            if let Ok(value) = reqwest::header::HeaderValue::from_str(&auth_value) {
                headers.insert("X-Emby-Authorization", value);
            }
        } else {
            let auth_value = format!("MediaBrowser Client=\"{}\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\"",
                                     self.client_name, self.client_name, self.device_id, self.client_version);

            if let Ok(value) = reqwest::header::HeaderValue::from_str(&auth_value) {
                headers.insert("X-Emby-Authorization", value);
            }
        }

        headers
    }

    /// Make a raw HTTP request
    ///
    /// This is the low-level method for making HTTP requests to the Jellyfin API.
    /// It handles URL building, header injection, and basic error handling.
    pub async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&[u8]>,
        content_type: Option<&str>,
    ) -> Result<Response> {
        let url = self.build_api_url(path)?;
        let headers = self.create_headers();

        debug!("Making {} request to: {}", method, url);

        let mut request_builder = self.http_client
            .request(method.clone(), &url)
            .headers(headers);

        // Add content type if specified
        if let Some(ct) = content_type {
            request_builder = request_builder.header("Content-Type", ct);
        }

        // Add body if provided
        if let Some(body_data) = body {
            request_builder = request_builder.body(body_data.to_vec());
        }

        let response = request_builder
            .send()
            .await
            .map_err(JellyfinError::Network)?;

        let status = response.status();
        debug!("Response status: {}", status);

        if status.is_success() {
            Ok(response)
        } else {
            let error = error_utils::response_to_error(response).await;
            error_utils::log_error(&error, &format!("API request to {}", path));
            Err(error.into())
        }
    }

    /// Make a JSON request
    ///
    /// This method handles JSON serialization/deserialization and is the
    /// primary method for making API requests.
    pub async fn request_json<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let body_bytes = if let Some(body_data) = body {
            Some(serde_json::to_vec(body_data).map_err(JellyfinError::Parsing)?)
        } else {
            None
        };

        let response = self.request_raw(
            method,
            path,
            body_bytes.as_deref(),
            Some("application/json"),
        ).await?;

        let response_text = response.text().await.map_err(JellyfinError::Network)?;

        if response_text.is_empty() {
            // Handle empty responses for requests that don't return data
            if std::any::type_name::<R>() == "()" {
                // This is a bit of a hack, but it works for unit type returns
                return Ok(serde_json::from_str("null").map_err(JellyfinError::Parsing)?);
            }
        }

        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            error!("Response text: {}", response_text);
            JellyfinError::Parsing(e).into()
        })
    }

    /// Make a GET request
    pub async fn get<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        self.request_json(Method::GET, path, None::<&()>).await
    }

    /// Make a POST request with JSON body
    pub async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        self.request_json(Method::POST, path, Some(body)).await
    }

    /// Make a PUT request with JSON body
    pub async fn put<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        self.request_json(Method::PUT, path, Some(body)).await
    }

    /// Make a DELETE request
    pub async fn delete<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        self.request_json(Method::DELETE, path, None::<&()>).await
    }

    // System API methods

    /// Get server information
    ///
    /// Retrieves detailed information about the Jellyfin server.
    /// Requires authentication.
    pub async fn get_server_info(&self) -> Result<crate::models::api::ServerInfo> {
        info!("Getting server information");
        self.get("/System/Info").await
    }

    /// Get public server information
    ///
    /// Retrieves basic server information without requiring authentication.
    /// This is useful for server discovery and initial connection validation.
    pub async fn get_public_server_info(&self, server_url: &str) -> Result<crate::models::api::PublicServerInfo> {
        info!("Getting public server information from: {}", server_url);

        // Create a temporary client for this request since we might not be connected yet
        let temp_url = self.normalize_server_url(server_url)?;
        let full_url = format!("{}/System/Info/Public", temp_url);

        let headers = self.create_headers();
        let response = self.http_client
            .get(&full_url)
            .headers(headers)
            .send()
            .await
            .map_err(JellyfinError::Network)?;

        let status = response.status();
        debug!("Public server info response status: {}", status);

        if status.is_success() {
            let response_text = response.text().await.map_err(JellyfinError::Network)?;
            serde_json::from_str(&response_text).map_err(|e| {
                error!("Failed to parse public server info JSON: {}", e);
                error!("Response text: {}", response_text);
                JellyfinError::Parsing(e).into()
            })
        } else {
            let error = error_utils::response_to_error(response).await;
            error_utils::log_error(&error, "get_public_server_info");
            Err(error.into())
        }
    }

    /// Ping the server
    ///
    /// Tests connectivity to the Jellyfin server. Returns success if the server
    /// is reachable and responding.
    pub async fn ping(&self) -> Result<()> {
        info!("Pinging server");

        let response = self.request_raw(Method::GET, "/System/Ping", None, None).await?;
        let response_text = response.text().await.map_err(JellyfinError::Network)?;

        debug!("Ping response: {}", response_text);
        Ok(())
    }

    /// Connect to a Jellyfin server
    ///
    /// Validates the server URL and retrieves public server information.
    /// This method should be called before attempting authentication.
    pub async fn connect(&mut self, server_url: &str) -> Result<crate::models::api::PublicServerInfo> {
        info!("Connecting to server: {}", server_url);

        // Normalize and validate the URL
        let normalized_url = self.normalize_server_url(server_url)?;

        // Test connectivity with ping
        let temp_url = self.server_url.clone();
        self.server_url = Some(normalized_url.clone());

        match self.ping().await {
            Ok(_) => {
                debug!("Server ping successful");
            }
            Err(e) => {
                warn!("Server ping failed, but continuing: {}", e);
                // Don't fail connection on ping failure, some servers might not support it
            }
        }

        // Get public server information
        let server_info = match self.get_public_server_info(&normalized_url).await {
            Ok(info) => {
                info!("Successfully connected to server: {} ({})", info.name, info.version);
                info
            }
            Err(e) => {
                error!("Failed to get public server info: {}", e);
                self.server_url = temp_url; // Restore previous URL on failure
                return Err(e);
            }
        };

        // Connection successful, keep the URL
        self.server_url = Some(normalized_url);
        Ok(server_info)
    }

    // Authentication API methods

    /// Authenticate with username and password
    ///
    /// Authenticates a user with the Jellyfin server using username and password.
    /// On successful authentication, the access token is automatically stored
    /// and will be included in subsequent API requests.
    pub async fn authenticate(&mut self, username: &str, password: &str) -> Result<crate::models::api::AuthResponse> {
        info!("Authenticating user: {}", username);

        if self.server_url.is_none() {
            return Err(JellyfinError::InvalidUrl("No server URL set. Call connect() first.".to_string()).into());
        }

        let auth_request = crate::models::api::AuthRequest::new(
            username.to_string(),
            password.to_string(),
        );

        match self.post::<_, crate::models::api::AuthResponse>("/Users/AuthenticateByName", &auth_request).await {
            Ok(auth_response) => {
                info!("Authentication successful for user: {}", auth_response.user.name);

                // Store the access token for future requests
                self.set_auth_token(auth_response.access_token.clone());

                debug!("Access token stored, user ID: {}", auth_response.user.id);
                Ok(auth_response)
            }
            Err(e) => {
                error!("Authentication failed for user {}: {}", username, e);
                // Clear any existing token on failed authentication
                self.clear_auth_token();
                Err(e)
            }
        }
    }

    /// Logout and clear authentication
    ///
    /// Clears the stored authentication token. This effectively logs out
    /// the user from the client, though it doesn't invalidate the token
    /// on the server side.
    pub fn logout(&mut self) {
        info!("Logging out user");
        self.clear_auth_token();
    }

    /// Check if a valid authentication token is available
    ///
    /// This only checks if a token is stored locally. It doesn't validate
    /// the token with the server.
    pub fn has_valid_token(&self) -> bool {
        self.auth_token.is_some()
    }

    /// Get current user information
    ///
    /// Retrieves information about the currently authenticated user.
    /// Requires authentication.
    pub async fn get_current_user(&self) -> Result<crate::models::api::UserInfo> {
        info!("Getting current user information");

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        self.get("/Users/Me").await
    }

    // Library API methods

    /// Get items from the library
    ///
    /// Retrieves items from the Jellyfin library based on query parameters.
    /// This is the primary method for browsing and filtering library content.
    /// Requires authentication.
    pub async fn get_items(&self, params: &crate::models::api::ItemsQuery) -> Result<crate::models::api::ItemsResponse> {
        info!("Getting items with query parameters");

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        // Build query string from parameters
        let query_string = params.to_query_string();
        let path = if query_string.is_empty() {
            "/Items".to_string()
        } else {
            format!("/Items?{}", query_string)
        };

        debug!("Items query path: {}", path);
        self.get(&path).await
    }

    /// Get a specific item by ID
    ///
    /// Retrieves detailed information about a specific library item.
    /// Requires authentication.
    pub async fn get_item(&self, item_id: &str) -> Result<crate::models::api::BaseItem> {
        info!("Getting item with ID: {}", item_id);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        if item_id.is_empty() {
            return Err(JellyfinError::InvalidUrl("Item ID cannot be empty".to_string()).into());
        }

        let path = format!("/Items/{}", item_id);
        self.get(&path).await
    }

    /// Search for items in the library
    ///
    /// Performs a search across the library using the search hints endpoint.
    /// This provides fast search results with basic item information.
    /// Requires authentication.
    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<crate::models::api::SearchHintResult> {
        info!("Searching for: {}", query);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        if query.is_empty() {
            return Err(JellyfinError::InvalidUrl("Search query cannot be empty".to_string()).into());
        }

        // URL encode the search term
        let encoded_query = urlencoding::encode(query);
        let mut path = format!("/Search/Hints?searchTerm={}", encoded_query);

        if let Some(limit_val) = limit {
            path.push_str(&format!("&limit={}", limit_val));
        }

        debug!("Search path: {}", path);
        self.get(&path).await
    }

    // Media streaming API methods

    /// Get playback info for an item
    ///
    /// Retrieves playback information including media sources and streaming details
    /// for a specific item. This is required before starting playback.
    /// Requires authentication.
    pub async fn get_playback_info(&self, item_id: &str, user_id: Option<&str>) -> Result<crate::models::api::PlaybackInfoResponse> {
        info!("Getting playback info for item: {}", item_id);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        if item_id.is_empty() {
            return Err(JellyfinError::InvalidUrl("Item ID cannot be empty".to_string()).into());
        }

        let mut path = format!("/Items/{}/PlaybackInfo", item_id);

        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }

        debug!("Playback info path: {}", path);
        self.get(&path).await
    }

    /// Get streaming URL for an item
    ///
    /// Generates a direct streaming URL for media playback. This method constructs
    /// the URL but doesn't make an HTTP request - the URL is meant to be used
    /// by media players for streaming.
    pub fn get_stream_url(&self, item_id: &str, params: &crate::models::api::StreamParams) -> Result<String> {
        info!("Generating stream URL for item: {}", item_id);

        let server_url = self.server_url
            .as_ref()
            .ok_or_else(|| JellyfinError::InvalidUrl("No server URL set".to_string()))?;

        if item_id.is_empty() {
            return Err(JellyfinError::InvalidUrl("Item ID cannot be empty".to_string()).into());
        }

        let query_string = params.to_query_string();
        let base_path = format!("/Videos/{}/stream", item_id);

        let url = if query_string.is_empty() {
            format!("{}{}", server_url, base_path)
        } else {
            format!("{}{}?{}", server_url, base_path, query_string)
        };

        debug!("Generated stream URL: {}", url);
        Ok(url)
    }

    /// Get streaming URL with container format
    ///
    /// Generates a streaming URL with a specific container format extension.
    /// This is useful for players that need specific file extensions.
    pub fn get_stream_url_with_container(&self, item_id: &str, container: &str, params: &crate::models::api::StreamParams) -> Result<String> {
        info!("Generating stream URL for item: {} with container: {}", item_id, container);

        let server_url = self.server_url
            .as_ref()
            .ok_or_else(|| JellyfinError::InvalidUrl("No server URL set".to_string()))?;

        if item_id.is_empty() {
            return Err(JellyfinError::InvalidUrl("Item ID cannot be empty".to_string()).into());
        }

        if container.is_empty() {
            return Err(JellyfinError::InvalidUrl("Container cannot be empty".to_string()).into());
        }

        let query_string = params.to_query_string();
        let base_path = format!("/Videos/{}/stream.{}", item_id, container);

        let url = if query_string.is_empty() {
            format!("{}{}", server_url, base_path)
        } else {
            format!("{}{}?{}", server_url, base_path, query_string)
        };

        debug!("Generated stream URL with container: {}", url);
        Ok(url)
    }

    /// Report playback start
    ///
    /// Reports to the server that playback has started for an item.
    /// This is used for tracking watch history and updating user data.
    /// Requires authentication.
    pub async fn report_playback_start(&self, info: &crate::models::api::PlaybackStartInfo) -> Result<()> {
        info!("Reporting playback start for item: {}", info.item_id);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        self.post::<_, serde_json::Value>("/Sessions/Playing", info).await?;
        debug!("Playback start reported successfully");
        Ok(())
    }

    /// Report playback progress
    ///
    /// Reports current playback progress to the server. This should be called
    /// periodically during playback to update the user's watch position.
    /// Requires authentication.
    pub async fn report_playback_progress(&self, info: &crate::models::api::PlaybackProgressInfo) -> Result<()> {
        info!("Reporting playback progress for item: {} at position: {}", info.item_id, info.position_ticks);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        self.post::<_, serde_json::Value>("/Sessions/Playing/Progress", info).await?;
        debug!("Playback progress reported successfully");
        Ok(())
    }

    /// Report playback stop
    ///
    /// Reports to the server that playback has stopped for an item.
    /// This finalizes the watch session and updates user data.
    /// Requires authentication.
    pub async fn report_playback_stop(&self, info: &crate::models::api::PlaybackStopInfo) -> Result<()> {
        info!("Reporting playback stop for item: {} at position: {}", info.item_id, info.position_ticks);

        if !self.is_authenticated() {
            return Err(JellyfinError::authentication("No authentication token available").into());
        }

        self.post::<_, serde_json::Value>("/Sessions/Playing/Stopped", info).await?;
        debug!("Playback stop reported successfully");
        Ok(())
    }

    /// Refresh authentication token
    ///
    /// Note: Jellyfin doesn't have a traditional token refresh endpoint.
    /// This method will re-authenticate using stored credentials if available,
    /// or return an error requiring manual re-authentication.
    pub async fn refresh_token(&mut self) -> Result<()> {
        warn!("Token refresh requested, but Jellyfin doesn't support token refresh");
        warn!("Manual re-authentication required");

        // Clear the current token since it's presumably invalid
        self.clear_auth_token();

        Err(JellyfinError::authentication(
            "Token refresh not supported. Please re-authenticate manually."
        ).into())
    }
}

impl Default for JellyfinClient {
    fn default() -> Self {
        Self::new()
    }
}