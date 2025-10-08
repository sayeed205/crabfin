//! Jellyfin API client implementation
//!
//! This module contains the main API client for communicating with Jellyfin servers.

use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info};

use super::http::{ApiError, ApiResponse, HttpClient};

/// Trait defining the Jellyfin API client interface
#[async_trait]
pub trait JellyfinApiClient {
    /// Authenticate with the Jellyfin server
    async fn authenticate(&self, username: &str, password: &str) -> Result<AuthenticationResult>;

    /// Get server information
    async fn get_server_info(&self) -> Result<ServerInfo>;

    /// Get user information
    async fn get_user_info(&self, user_id: &str, access_token: &str) -> Result<UserInfo>;

    /// Make a generic API request
    async fn request<T, R>(&self, method: Method, path: &str, body: Option<T>, access_token: Option<&str>) -> Result<R>
    where
        T: Serialize + Send,
        R: for<'de> Deserialize<'de>;
}

/// Basic implementation of the Jellyfin API client
#[derive(Clone)]
pub struct JellyfinClient {
    http_client: HttpClient,
}

impl JellyfinClient {
    /// Create a new Jellyfin API client
    pub fn new(server_url: String) -> Result<Self> {
        let http_client = HttpClient::new(server_url)?;
        Ok(Self { http_client })
    }

    /// Get the server URL
    pub fn server_url(&self) -> &str {
        self.http_client.base_url()
    }
}

#[async_trait]
impl JellyfinApiClient for JellyfinClient {
    async fn authenticate(&self, username: &str, password: &str) -> Result<AuthenticationResult> {
        debug!("Authenticating user: {}", username);

        let auth_request = AuthenticationRequest {
            username: username.to_string(),
            pw: password.to_string(),
        };

        let response: AuthenticationResult = self
            .request(Method::POST, "/Users/authenticatebyname", Some(auth_request), None)
            .await?;

        info!("Authentication successful for user: {}", username);
        Ok(response)
    }

    async fn get_server_info(&self) -> Result<ServerInfo> {
        debug!("Fetching server information");

        let response: ServerInfo = self
            .request(Method::GET, "/System/Info/Public", None::<()>, None)
            .await?;

        Ok(response)
    }

    async fn get_user_info(&self, user_id: &str, access_token: &str) -> Result<UserInfo> {
        debug!("Fetching user information for user: {}", user_id);

        let path = format!("/Users/{}", user_id);
        let response: UserInfo = self
            .request(Method::GET, &path, None::<()>, Some(access_token))
            .await?;

        Ok(response)
    }

    async fn request<T, R>(&self, method: Method, path: &str, body: Option<T>, access_token: Option<&str>) -> Result<R>
    where
        T: Serialize + Send,
        R: for<'de> Deserialize<'de>,
    {
        let url = self.http_client.build_url(path);
        debug!("Making {} request to: {}", method, url);

        let mut request_builder = self.http_client.client().request(method, &url);

        // Add authorization header if token is provided
        if let Some(token) = access_token {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        // Add content type for requests with body
        if body.is_some() {
            request_builder = request_builder.header("Content-Type", "application/json");
        }

        // Add body if provided
        if let Some(body_data) = body {
            request_builder = request_builder.json(&body_data);
        }

        let response = request_builder.send().await?;
        let status = response.status();

        if status.is_success() {
            let data: R = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("API request failed with status {}: {}", status, error_text);

            let api_error = ApiError {
                message: format!("Request failed with status {}", status),
                code: Some(status.to_string()),
                details: serde_json::from_str(&error_text).ok(),
            };

            Err(anyhow::Error::new(api_error))
        }
    }
}

/// Authentication request structure
#[derive(Debug, Serialize)]
struct AuthenticationRequest {
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Pw")]
    pw: String,
}

/// Authentication response structure
#[derive(Debug, Deserialize)]
pub struct AuthenticationResult {
    #[serde(rename = "User")]
    pub user: UserInfo,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
}

/// Server information structure
#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: Option<String>,
}

/// User information structure
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServerId")]
    pub server_id: Option<String>,
    #[serde(rename = "HasPassword")]
    pub has_password: Option<bool>,
    #[serde(rename = "HasConfiguredPassword")]
    pub has_configured_password: Option<bool>,
}