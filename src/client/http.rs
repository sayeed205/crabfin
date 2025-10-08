//! HTTP client utilities
//!
//! This module contains HTTP client configuration and utilities for API requests.

use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP client wrapper for Jellyfin API requests
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new(base_url: String) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("Crabfin/0.1.0")
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            timeout: Duration::from_secs(30),
        })
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(base_url: String, timeout: Duration) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(timeout)
            .user_agent("Crabfin/0.1.0")
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            timeout,
        })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the configured timeout
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Get the underlying reqwest client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Build a full URL from a path
    pub fn build_url(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        format!("{}/{}", self.base_url, path)
    }
}

/// Request/Response type foundations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest<T> {
    pub data: T,
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
}

/// Error response from Jellyfin API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.message)
    }
}

impl std::error::Error for ApiError {}