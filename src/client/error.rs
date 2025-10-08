use serde::{Deserialize, Serialize};
use std::fmt;

/// Jellyfin-specific error types for API client operations
#[derive(Debug, thiserror::Error)]
pub enum JellyfinError {
    /// Network-related errors from reqwest
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    /// Authentication failed with the server
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    /// Server returned an error response
    #[error("Server error: {status} - {message}")]
    Server { status: u16, message: String },
    
    /// Failed to parse JSON response
    #[error("Parsing error: {0}")]
    Parsing(#[from] serde_json::Error),
    
    /// Invalid server URL provided
    #[error("Invalid server URL: {0}")]
    InvalidUrl(String),
    
    /// Server is not reachable or incompatible
    #[error("Server unreachable or incompatible: {0}")]
    ServerUnreachable(String),
    
    /// Required field missing from response
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    /// Invalid response format
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    
    /// Connection timeout
    #[error("Connection timeout: {0}")]
    Timeout(String),
    
    /// SSL/TLS certificate error
    #[error("Certificate error: {0}")]
    Certificate(String),
    
    /// Generic error from anyhow
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// Standard Jellyfin API error response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JellyfinApiError {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    
    #[serde(rename = "Description")]
    pub description: Option<String>,
    
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<String>,
    
    #[serde(rename = "StackTrace")]
    pub stack_trace: Option<String>,
}

impl fmt::Display for JellyfinApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
            if let Some(desc) = &self.description {
                write!(f, ": {}", desc)?;
            }
        } else if let Some(desc) = &self.description {
            write!(f, "{}", desc)?;
        } else {
            write!(f, "Unknown server error")?;
        }
        Ok(())
    }
}

impl JellyfinError {
    /// Create a server error from HTTP status and response body
    pub fn from_response(status: u16, body: &str) -> Self {
        // Try to parse as Jellyfin API error format
        if let Ok(api_error) = serde_json::from_str::<JellyfinApiError>(body) {
            JellyfinError::Server {
                status,
                message: api_error.to_string(),
            }
        } else {
            // Fallback to raw body
            JellyfinError::Server {
                status,
                message: body.to_string(),
            }
        }
    }
    
    /// Create an authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        JellyfinError::Authentication(message.into())
    }
    
    /// Create an invalid URL error
    pub fn invalid_url<S: Into<String>>(url: S) -> Self {
        JellyfinError::InvalidUrl(url.into())
    }
    
    /// Create a server unreachable error
    pub fn server_unreachable<S: Into<String>>(message: S) -> Self {
        JellyfinError::ServerUnreachable(message.into())
    }
    
    /// Create a missing field error
    pub fn missing_field<S: Into<String>>(field: S) -> Self {
        JellyfinError::MissingField(field.into())
    }
    
    /// Create an invalid response error
    pub fn invalid_response<S: Into<String>>(message: S) -> Self {
        JellyfinError::InvalidResponse(message.into())
    }
    
    /// Create a timeout error
    pub fn timeout<S: Into<String>>(message: S) -> Self {
        JellyfinError::Timeout(message.into())
    }
    
    /// Create a certificate error
    pub fn certificate<S: Into<String>>(message: S) -> Self {
        JellyfinError::Certificate(message.into())
    }
    
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            JellyfinError::Network(err) => {
                // Retry on timeout, connection errors, but not on client errors
                err.is_timeout() || err.is_connect()
            }
            JellyfinError::Server { status, .. } => {
                // Retry on 5xx server errors, but not 4xx client errors
                *status >= 500
            }
            JellyfinError::Timeout(_) => true,
            JellyfinError::ServerUnreachable(_) => true,
            JellyfinError::Internal(_) => false,
            _ => false,
        }
    }
    
    /// Check if this error indicates authentication is required
    pub fn requires_authentication(&self) -> bool {
        match self {
            JellyfinError::Authentication(_) => true,
            JellyfinError::Server { status, .. } => *status == 401,
            JellyfinError::Internal(_) => false,
            _ => false,
        }
    }
    
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            JellyfinError::Network(err) => {
                if err.is_timeout() {
                    "Connection timed out. Please check your network connection.".to_string()
                } else if err.is_connect() {
                    "Unable to connect to the server. Please check the server URL and your network connection.".to_string()
                } else {
                    "Network error occurred. Please try again.".to_string()
                }
            }
            JellyfinError::Authentication(_) => {
                "Authentication failed. Please check your username and password.".to_string()
            }
            JellyfinError::Server { status, message } => {
                match *status {
                    400 => "Bad request. Please check your input.".to_string(),
                    401 => "Authentication required. Please log in.".to_string(),
                    403 => "Access denied. You don't have permission to access this resource.".to_string(),
                    404 => "The requested resource was not found.".to_string(),
                    500..=599 => "Server error occurred. Please try again later.".to_string(),
                    _ => format!("Server returned error ({}): {}", status, message),
                }
            }
            JellyfinError::Parsing(_) => {
                "Failed to parse server response. The server may be incompatible.".to_string()
            }
            JellyfinError::InvalidUrl(_) => {
                "Invalid server URL. Please check the URL format.".to_string()
            }
            JellyfinError::ServerUnreachable(_) => {
                "Server is unreachable. Please check the server URL and your network connection.".to_string()
            }
            JellyfinError::MissingField(field) => {
                format!("Server response is missing required field: {}", field)
            }
            JellyfinError::InvalidResponse(_) => {
                "Server returned an invalid response format.".to_string()
            }
            JellyfinError::Timeout(_) => {
                "Request timed out. Please try again.".to_string()
            }
            JellyfinError::Certificate(_) => {
                "SSL certificate error. Please check the server's certificate.".to_string()
            }
            JellyfinError::Internal(err) => {
                format!("Internal error: {}", err)
            }
        }
    }
}

/// Result type alias for Jellyfin API operations
pub type JellyfinResult<T> = Result<T, JellyfinError>;

/// Utility functions for error handling in HTTP responses
pub struct ErrorUtils;

impl ErrorUtils {
    /// Extract error information from HTTP response
    pub async fn extract_error_from_response(response: reqwest::Response) -> JellyfinError {
        let status = response.status().as_u16();
        
        match response.text().await {
            Ok(body) => JellyfinError::from_response(status, &body),
            Err(err) => JellyfinError::Network(err),
        }
    }
    
    /// Check if HTTP status indicates success
    pub fn is_success_status(status: u16) -> bool {
        (200..300).contains(&status)
    }
    
    /// Check if HTTP status indicates client error
    pub fn is_client_error(status: u16) -> bool {
        (400..500).contains(&status)
    }
    
    /// Check if HTTP status indicates server error
    pub fn is_server_error(status: u16) -> bool {
        (500..600).contains(&status)
    }
    
    /// Convert reqwest error to JellyfinError with context
    pub fn convert_reqwest_error(err: reqwest::Error, context: &str) -> JellyfinError {
        if err.is_timeout() {
            JellyfinError::timeout(format!("{}: {}", context, err))
        } else if err.is_connect() {
            JellyfinError::server_unreachable(format!("{}: {}", context, err))
        } else {
            JellyfinError::Network(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jellyfin_error_creation() {
        let auth_error = JellyfinError::authentication("Invalid credentials");
        assert!(matches!(auth_error, JellyfinError::Authentication(_)));
        assert!(auth_error.requires_authentication());
        
        let server_error = JellyfinError::Server {
            status: 500,
            message: "Internal server error".to_string(),
        };
        assert!(server_error.is_retryable());
    }

    #[test]
    fn test_error_retryability() {
        let timeout_error = JellyfinError::timeout("Connection timeout");
        assert!(timeout_error.is_retryable());
        
        let auth_error = JellyfinError::authentication("Invalid token");
        assert!(!auth_error.is_retryable());
        assert!(auth_error.requires_authentication());
        
        let client_error = JellyfinError::Server {
            status: 400,
            message: "Bad request".to_string(),
        };
        assert!(!client_error.is_retryable());
        
        let server_error = JellyfinError::Server {
            status: 503,
            message: "Service unavailable".to_string(),
        };
        assert!(server_error.is_retryable());
    }

    #[test]
    fn test_user_messages() {
        let auth_error = JellyfinError::authentication("Token expired");
        let message = auth_error.user_message();
        assert!(message.contains("Authentication failed"));
        
        let network_error = JellyfinError::invalid_url("not-a-url");
        let message = network_error.user_message();
        assert!(message.contains("Invalid server URL"));
    }

    #[test]
    fn test_api_error_parsing() {
        let json = r#"{"Name":"ArgumentException","Description":"Invalid argument","ErrorCode":"ARG001"}"#;
        let api_error: JellyfinApiError = serde_json::from_str(json).unwrap();
        assert_eq!(api_error.name, Some("ArgumentException".to_string()));
        assert_eq!(api_error.description, Some("Invalid argument".to_string()));
        
        let error = JellyfinError::from_response(400, json);
        if let JellyfinError::Server { status, message } = error {
            assert_eq!(status, 400);
            assert!(message.contains("ArgumentException"));
        } else {
            panic!("Expected server error");
        }
    }
}