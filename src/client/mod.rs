use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP client configuration for Jellyfin API
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub user_agent: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            timeout: Duration::from_secs(30),
            user_agent: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        }
    }
}

/// Basic HTTP client wrapper for Jellyfin API communication
#[derive(Debug, Clone)]
pub struct JellyfinClient {
    client: Client,
    config: ClientConfig,
}

impl JellyfinClient {
    /// Create a new Jellyfin client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self { client, config })
    }

    /// Get the base URL for this client
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Get the underlying HTTP client
    pub fn http_client(&self) -> &Client {
        &self.client
    }
}

/// Trait for Jellyfin API operations
pub trait JellyfinApi {
    /// Perform a GET request to the specified endpoint
    async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>;

    /// Perform a POST request to the specified endpoint
    async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize;

    /// Check server connectivity
    async fn ping(&self) -> Result<()>;
}

/// Basic request wrapper for API calls
#[derive(Debug, Serialize)]
pub struct ApiRequest<T> {
    pub data: T,
}

/// Basic response wrapper for API responses
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Server information response
#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Id")]
    pub id: String,
}