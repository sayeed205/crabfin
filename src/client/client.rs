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
}

impl Default for JellyfinClient {
    fn default() -> Self {
        Self::new()
    }
}