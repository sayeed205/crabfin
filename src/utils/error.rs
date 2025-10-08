//! Error handling utilities
//!
//! This module contains error types and error handling utilities.

use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// Initialization errors
    #[error("Initialization error: {0}")]
    InitializationError(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Network/API errors
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// UI/GPUI errors
    #[error("UI error: {0}")]
    UiError(String),

    /// Service errors
    #[error("Service error: {0}")]
    ServiceError(String),

    /// IO errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// HTTP request errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Generic errors
    #[error("Generic error: {0}")]
    Generic(String),
}

impl AppError {
    /// Create a new initialization error
    pub fn initialization<S: Into<String>>(msg: S) -> Self {
        Self::InitializationError(msg.into())
    }

    /// Create a new configuration error
    pub fn configuration<S: Into<String>>(msg: S) -> Self {
        Self::ConfigurationError(msg.into())
    }

    /// Create a new network error
    pub fn network<S: Into<String>>(msg: S) -> Self {
        Self::NetworkError(msg.into())
    }

    /// Create a new authentication error
    pub fn authentication<S: Into<String>>(msg: S) -> Self {
        Self::AuthenticationError(msg.into())
    }

    /// Create a new UI error
    pub fn ui<S: Into<String>>(msg: S) -> Self {
        Self::UiError(msg.into())
    }

    /// Create a new service error
    pub fn service<S: Into<String>>(msg: S) -> Self {
        Self::ServiceError(msg.into())
    }

    /// Create a generic error
    pub fn generic<S: Into<String>>(msg: S) -> Self {
        Self::Generic(msg.into())
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            AppError::InitializationError(_) => {
                "Failed to initialize the application. Please try restarting.".to_string()
            }
            AppError::ConfigurationError(_) => {
                "Configuration error. Please check your settings.".to_string()
            }
            AppError::NetworkError(_) => {
                "Network connection error. Please check your internet connection.".to_string()
            }
            AppError::AuthenticationError(_) => {
                "Authentication failed. Please check your credentials.".to_string()
            }
            AppError::UiError(_) => {
                "Interface error occurred. Please try again.".to_string()
            }
            AppError::ServiceError(_) => {
                "Service error occurred. Please try again.".to_string()
            }
            AppError::IoError(_) => {
                "File system error occurred. Please check file permissions.".to_string()
            }
            AppError::JsonError(_) => {
                "Data format error occurred. Please try again.".to_string()
            }
            AppError::HttpError(_) => {
                "Network request failed. Please check your connection.".to_string()
            }
            AppError::Generic(msg) => msg.clone(),
        }
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            AppError::InitializationError(_) => false,
            AppError::ConfigurationError(_) => true,
            AppError::NetworkError(_) => true,
            AppError::AuthenticationError(_) => true,
            AppError::UiError(_) => true,
            AppError::ServiceError(_) => true,
            AppError::IoError(_) => true,
            AppError::JsonError(_) => true,
            AppError::HttpError(_) => true,
            AppError::Generic(_) => true,
        }
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Error propagation helper trait
pub trait ErrorContext<T> {
    /// Add context to an error
    fn with_context<F>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String;

    /// Add static context to an error
    fn context(self, msg: &'static str) -> AppResult<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn with_context<F>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            let context = f();
            AppError::generic(format!("{}: {}", context, base_error))
        })
    }

    fn context(self, msg: &'static str) -> AppResult<T> {
        self.with_context(|| msg.to_string())
    }
}

/// Log and return user-friendly error
pub fn handle_error(error: &AppError) -> String {
    tracing::error!("Application error: {:#}", error);
    error.user_message()
}