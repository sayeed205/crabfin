//! Error handling types and utilities for the Crabfin API client
//!
//! This module defines comprehensive error types for all possible failure modes
//! when communicating with servers, including network errors, authentication
//! failures, server errors, and parsing errors.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Comprehensive error type for Crabfin API operations
///
/// This enum covers all possible error conditions that can occur when
/// communicating with a server, providing detailed error information
/// and appropriate error handling strategies.
#[derive(Debug, Error)]
pub enum CrabfinError {
    /// Network-related errors (connection failures, timeouts, etc.)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Authentication and authorization failures
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// Server-side errors with HTTP status codes
    #[error("Server error: {status} - {message}")]
    Server { status: u16, message: String },

    /// JSON parsing and serialization errors
    #[error("Parsing error: {0}")]
    Parsing(#[from] serde_json::Error),

    /// Invalid server URL or malformed URLs
    #[error("Invalid server URL: {0}")]
    InvalidUrl(String),

    /// API rate limiting errors
    #[error("Rate limited: {message}")]
    RateLimit { message: String, retry_after: Option<u64> },

    /// Server is unavailable or unreachable
    #[error("Server unavailable: {message}")]
    ServerUnavailable { message: String },

    /// Invalid API response format
    #[error("Invalid response format: {message}")]
    InvalidResponse { message: String },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Generic API errors with optional error codes
    #[error("API error: {message}")]
    Api { message: String, code: Option<String> },
}

impl CrabfinError {
    /// Create a new authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a new server error
    pub fn server(status: u16, message: String) -> Self {
        Self::Server { status, message }
    }

    /// Create a new rate limit error
    pub fn rate_limit<S: Into<String>>(message: S, retry_after: Option<u64>) -> Self {
        Self::RateLimit {
            message: message.into(),
            retry_after,
        }
    }

    /// Create a new server unavailable error
    pub fn server_unavailable<S: Into<String>>(message: S) -> Self {
        Self::ServerUnavailable {
            message: message.into(),
        }
    }

    /// Create a new invalid response error
    pub fn invalid_response<S: Into<String>>(message: S) -> Self {
        Self::InvalidResponse {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration(message.into())
    }

    /// Create a new API error
    pub fn api<S: Into<String>>(message: S, code: Option<String>) -> Self {
        Self::Api {
            message: message.into(),
            code,
        }
    }

    /// Check if this error is retryable
    ///
    /// Returns true if the operation that caused this error can be safely retried.
    pub fn is_retryable(&self) -> bool {
        match self {
            // Network errors are generally retryable
            Self::Network(e) => {
                // Timeout errors are retryable
                if e.is_timeout() {
                    return true;
                }
                // Connection errors are retryable
                if e.is_connect() {
                    return true;
                }
                // Request errors are generally not retryable
                false
            }
            // Server errors: 5xx are retryable, 4xx are not
            Self::Server { status, .. } => *status >= 500,
            // Rate limit errors are retryable after waiting
            Self::RateLimit { .. } => true,
            // Server unavailable errors are retryable
            Self::ServerUnavailable { .. } => true,
            // Other errors are generally not retryable
            _ => false,
        }
    }

    /// Get the retry delay in seconds for retryable errors
    ///
    /// Returns the recommended delay before retrying the operation.
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            Self::RateLimit { retry_after, .. } => *retry_after,
            Self::Network(_) => Some(1), // 1 second for network errors
            Self::Server { status, .. } if *status >= 500 => Some(2), // 2 seconds for server errors
            Self::ServerUnavailable { .. } => Some(5), // 5 seconds for unavailable servers
            _ => None,
        }
    }

    /// Check if this is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, Self::Authentication { .. })
    }

    /// Check if this is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::Network(_))
    }

    /// Check if this is a server error
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Server { .. })
    }

    /// Get the HTTP status code if this is a server error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Server { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// API error response structure
///
/// This represents the standard error response format from servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Error message from the server
    #[serde(rename = "Message")]
    pub message: Option<String>,

    /// Error code from the server
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<String>,

    /// Additional error details
    #[serde(flatten)]
    pub details: serde_json::Value,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)
        } else {
            write!(f, "Unknown API error")
        }
    }
}

/// Result type alias for Crabfin operations
pub type CrabfinResult<T> = Result<T, CrabfinError>;

/// Utility functions for error handling
pub mod utils {
    use super::*;
    use reqwest::Response;
    use tracing::{error, warn};

    /// Convert an HTTP response to a CrabfinError
    ///
    /// This function examines the response status and attempts to parse
    /// the response body as a error response.
    pub async fn response_to_error(response: Response) -> CrabfinError {
        let status = response.status();
        let status_code = status.as_u16();

        // Try to get the response text
        let response_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                error!("Failed to read error response body: {}", e);
                return CrabfinError::server(status_code, "Failed to read error response".to_string());
            }
        };

        // Try to parse as API error response
        if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&response_text) {
            let message = api_error.message.unwrap_or_else(|| "Unknown API error".to_string());

            return match status_code {
                401 => CrabfinError::authentication(message),
                429 => {
                    // Try to extract retry-after from the error details
                    let retry_after = api_error.details
                        .get("RetryAfter")
                        .and_then(|v| v.as_u64());
                    CrabfinError::rate_limit(message, retry_after)
                }
                500..=599 => CrabfinError::server_unavailable(message),
                _ => CrabfinError::server(status_code, message),
            };
        }

        // Fallback to generic server error
        let message = if response_text.is_empty() {
            format!("HTTP {}", status)
        } else {
            response_text
        };

        match status_code {
            401 => CrabfinError::authentication(message),
            429 => CrabfinError::rate_limit(message, None),
            500..=599 => CrabfinError::server_unavailable(message),
            _ => CrabfinError::server(status_code, message),
        }
    }

    /// Log an error with appropriate level based on error type
    pub fn log_error(error: &CrabfinError, context: &str) {
        match error {
            CrabfinError::Network(e) if e.is_timeout() => {
                warn!("{}: Network timeout - {}", context, error);
            }
            CrabfinError::Network(_) => {
                error!("{}: Network error - {}", context, error);
            }
            CrabfinError::Authentication { .. } => {
                warn!("{}: Authentication error - {}", context, error);
            }
            CrabfinError::Server { status, .. } if *status >= 500 => {
                error!("{}: Server error - {}", context, error);
            }
            CrabfinError::Server { .. } => {
                warn!("{}: Client error - {}", context, error);
            }
            CrabfinError::RateLimit { .. } => {
                warn!("{}: Rate limited - {}", context, error);
            }
            _ => {
                error!("{}: Error - {}", context, error);
            }
        }
    }

    /// Check if an error indicates the server is unreachable
    pub fn is_server_unreachable(error: &CrabfinError) -> bool {
        match error {
            CrabfinError::Network(e) => e.is_connect() || e.is_timeout(),
            CrabfinError::ServerUnavailable { .. } => true,
            CrabfinError::Server { status, .. } => *status >= 500,
            _ => false,
        }
    }

    /// Extract authentication error details
    pub fn extract_auth_error_details(error: &CrabfinError) -> Option<String> {
        match error {
            CrabfinError::Authentication { message } => Some(message.clone()),
            CrabfinError::Server { status: 401, message } => Some(message.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryability() {
        // Network errors should be retryable (using a mock network error)
        // Note: In real usage, reqwest::Error would be created by reqwest itself
        let network_error = CrabfinError::server_unavailable("Connection timeout");
        assert!(network_error.is_retryable());

        // 5xx server errors should be retryable
        let server_error = CrabfinError::server(500, "Internal server error".to_string());
        assert!(server_error.is_retryable());

        // 4xx client errors should not be retryable
        let client_error = CrabfinError::server(400, "Bad request".to_string());
        assert!(!client_error.is_retryable());

        // Authentication errors should not be retryable
        let auth_error = CrabfinError::authentication("Invalid credentials");
        assert!(!auth_error.is_retryable());

        // Rate limit errors should be retryable
        let rate_limit_error = CrabfinError::rate_limit("Too many requests", Some(60));
        assert!(rate_limit_error.is_retryable());
    }

    #[test]
    fn test_retry_delay() {
        // Rate limit error with retry-after
        let rate_limit_error = CrabfinError::rate_limit("Too many requests", Some(60));
        assert_eq!(rate_limit_error.retry_delay(), Some(60));

        // Server error should have default delay
        let server_error = CrabfinError::server(500, "Internal server error".to_string());
        assert_eq!(server_error.retry_delay(), Some(2));

        // Non-retryable error should have no delay
        let auth_error = CrabfinError::authentication("Invalid credentials");
        assert_eq!(auth_error.retry_delay(), None);
    }

    #[test]
    fn test_error_type_checks() {
        let auth_error = CrabfinError::authentication("Invalid credentials");
        assert!(auth_error.is_authentication_error());
        assert!(!auth_error.is_network_error());
        assert!(!auth_error.is_server_error());

        let server_error = CrabfinError::server(500, "Internal server error".to_string());
        assert!(!server_error.is_authentication_error());
        assert!(!server_error.is_network_error());
        assert!(server_error.is_server_error());
        assert_eq!(server_error.status_code(), Some(500));
    }
}