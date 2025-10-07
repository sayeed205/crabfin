// Error handling infrastructure for Crabfin

use anyhow::Result;
use std::fmt;
use tracing::error;

/// Application-specific error types
#[derive(Debug, Clone)]
pub enum AppError {
    /// Network-related errors (connection, timeout, server errors)
    Network(NetworkError),
    /// Authentication errors (invalid credentials, expired tokens)
    Authentication(AuthError),
    /// Data parsing errors (malformed responses)
    DataParsing(DataError),
    /// UI errors (component failures, state inconsistencies)
    UserInterface(UiError),
    /// Configuration errors (invalid settings, missing files)
    Configuration(ConfigError),
}

/// Network error variants
#[derive(Debug, Clone)]
pub enum NetworkError {
    ConnectionFailed(String),
    Timeout(String),
    ServerError { status: u16, message: String },
    InvalidUrl(String),
    RequestFailed(String),
}

/// Authentication error variants
#[derive(Debug, Clone)]
pub enum AuthError {
    InvalidCredentials,
    TokenExpired,
    TokenInvalid,
    ServerUnauthorized(String),
    UserNotFound,
    AuthenticationRequired,
}

/// Data parsing error variants
#[derive(Debug, Clone)]
pub enum DataError {
    JsonParseError(String),
    MissingField(String),
    InvalidFormat(String),
    UnexpectedResponse(String),
}

/// UI error variants
#[derive(Debug, Clone)]
pub enum UiError {
    ComponentInitializationFailed(String),
    StateInconsistency(String),
    RenderingError(String),
    WindowCreationFailed(String),
}

/// Configuration error variants
#[derive(Debug, Clone)]
pub enum ConfigError {
    FileNotFound(String),
    InvalidFormat(String),
    PermissionDenied(String),
    WriteFailed(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Network(err) => write!(f, "Network error: {}", err),
            AppError::Authentication(err) => write!(f, "Authentication error: {}", err),
            AppError::DataParsing(err) => write!(f, "Data parsing error: {}", err),
            AppError::UserInterface(err) => write!(f, "UI error: {}", err),
            AppError::Configuration(err) => write!(f, "Configuration error: {}", err),
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed(url) => {
                write!(f, "Failed to connect to server: {}", url)
            }
            NetworkError::Timeout(url) => {
                write!(f, "Request timed out for: {}", url)
            }
            NetworkError::ServerError { status, message } => {
                write!(f, "Server error ({}): {}", status, message)
            }
            NetworkError::InvalidUrl(url) => {
                write!(f, "Invalid URL: {}", url)
            }
            NetworkError::RequestFailed(reason) => {
                write!(f, "Request failed: {}", reason)
            }
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => {
                write!(f, "Invalid username or password")
            }
            AuthError::TokenExpired => {
                write!(f, "Authentication token has expired. Please log in again")
            }
            AuthError::TokenInvalid => {
                write!(f, "Authentication token is invalid")
            }
            AuthError::ServerUnauthorized(server) => {
                write!(f, "Unauthorized access to server: {}", server)
            }
            AuthError::UserNotFound => {
                write!(f, "User not found on the server")
            }
            AuthError::AuthenticationRequired => {
                write!(f, "Authentication is required to access this resource")
            }
        }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::JsonParseError(details) => {
                write!(f, "Failed to parse JSON response: {}", details)
            }
            DataError::MissingField(field) => {
                write!(f, "Required field '{}' is missing from response", field)
            }
            DataError::InvalidFormat(details) => {
                write!(f, "Data format is invalid: {}", details)
            }
            DataError::UnexpectedResponse(details) => {
                write!(f, "Received unexpected response: {}", details)
            }
        }
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::ComponentInitializationFailed(component) => {
                write!(f, "Failed to initialize UI component: {}", component)
            }
            UiError::StateInconsistency(details) => {
                write!(f, "UI state inconsistency detected: {}", details)
            }
            UiError::RenderingError(details) => {
                write!(f, "UI rendering error: {}", details)
            }
            UiError::WindowCreationFailed(details) => {
                write!(f, "Failed to create window: {}", details)
            }
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}", path)
            }
            ConfigError::InvalidFormat(details) => {
                write!(f, "Invalid configuration format: {}", details)
            }
            ConfigError::PermissionDenied(path) => {
                write!(f, "Permission denied accessing: {}", path)
            }
            ConfigError::WriteFailed(path) => {
                write!(f, "Failed to write configuration file: {}", path)
            }
        }
    }
}

impl std::error::Error for AppError {}
impl std::error::Error for NetworkError {}
impl std::error::Error for AuthError {}
impl std::error::Error for DataError {}
impl std::error::Error for UiError {}
impl std::error::Error for ConfigError {}

/// Error handling utilities and helper functions
pub struct ErrorHandler;

impl ErrorHandler {
    /// Log and convert an error to a user-friendly message
    pub fn handle_error(error: &anyhow::Error) -> String {
        error!("Application error occurred: {:?}", error);

        // Try to downcast to our custom error types for better user messages
        if let Some(app_error) = error.downcast_ref::<AppError>() {
            Self::format_user_message(app_error)
        } else {
            // Fallback for other error types
            "An unexpected error occurred. Please try again.".to_string()
        }
    }

    /// Format user-friendly error messages
    fn format_user_message(error: &AppError) -> String {
        match error {
            AppError::Network(NetworkError::ConnectionFailed(_)) => {
                "Unable to connect to the Jellyfin server. Please check your network connection and server URL.".to_string()
            }
            AppError::Network(NetworkError::Timeout(_)) => {
                "The request timed out. Please check your network connection and try again.".to_string()
            }
            AppError::Network(NetworkError::ServerError { status, .. }) => {
                match *status {
                    404 => "The requested resource was not found on the server.".to_string(),
                    500..=599 => "The server encountered an error. Please try again later.".to_string(),
                    _ => "The server returned an error. Please try again.".to_string(),
                }
            }
            AppError::Network(NetworkError::InvalidUrl(_)) => {
                "The server URL is invalid. Please check the URL format and try again.".to_string()
            }
            AppError::Network(NetworkError::RequestFailed(_)) => {
                "The network request failed. Please check your connection and try again.".to_string()
            }
            AppError::Authentication(AuthError::InvalidCredentials) => {
                "Invalid username or password. Please check your credentials and try again.".to_string()
            }
            AppError::Authentication(AuthError::TokenExpired) => {
                "Your session has expired. Please log in again.".to_string()
            }
            AppError::Authentication(_) => {
                "Authentication failed. Please check your credentials and try again.".to_string()
            }
            AppError::DataParsing(_) => {
                "Failed to process server response. The server may be incompatible or experiencing issues.".to_string()
            }
            AppError::UserInterface(_) => {
                "A user interface error occurred. Please restart the application.".to_string()
            }
            AppError::Configuration(_) => {
                "Configuration error. Please check your settings and try again.".to_string()
            }
        }
    }

    /// Create a network error with context
    pub fn network_error(error: NetworkError, context: &str) -> anyhow::Error {
        anyhow::Error::new(AppError::Network(error)).context(context.to_string())
    }

    /// Create an authentication error with context
    pub fn auth_error(error: AuthError, context: &str) -> anyhow::Error {
        anyhow::Error::new(AppError::Authentication(error)).context(context.to_string())
    }

    /// Create a data parsing error with context
    pub fn data_error(error: DataError, context: &str) -> anyhow::Error {
        anyhow::Error::new(AppError::DataParsing(error)).context(context.to_string())
    }

    /// Create a UI error with context
    pub fn ui_error(error: UiError, context: &str) -> anyhow::Error {
        anyhow::Error::new(AppError::UserInterface(error)).context(context.to_string())
    }

    /// Create a configuration error with context
    pub fn config_error(error: ConfigError, context: &str) -> anyhow::Error {
        anyhow::Error::new(AppError::Configuration(error)).context(context.to_string())
    }
}

/// Result type alias for convenience
pub type AppResult<T> = Result<T>;

/// Trait for converting errors to user-friendly messages
pub trait UserFriendlyError {
    fn user_message(&self) -> String;
}

impl UserFriendlyError for anyhow::Error {
    fn user_message(&self) -> String {
        ErrorHandler::handle_error(self)
    }
}

/// Macro for creating context-aware errors
#[macro_export]
macro_rules! app_error {
    ($error_type:expr, $context:expr) => {
        anyhow::Error::new($error_type).context($context)
    };
}

/// Macro for network errors
#[macro_export]
macro_rules! network_error {
    ($error:expr, $context:expr) => {
        $crate::error::ErrorHandler::network_error($error, $context)
    };
}

/// Macro for authentication errors
#[macro_export]
macro_rules! auth_error {
    ($error:expr, $context:expr) => {
        $crate::error::ErrorHandler::auth_error($error, $context)
    };
}

/// Macro for data parsing errors
#[macro_export]
macro_rules! data_error {
    ($error:expr, $context:expr) => {
        $crate::error::ErrorHandler::data_error($error, $context)
    };
}

/// Macro for UI errors
#[macro_export]
macro_rules! ui_error {
    ($error:expr, $context:expr) => {
        $crate::error::ErrorHandler::ui_error($error, $context)
    };
}

/// Macro for configuration errors
#[macro_export]
macro_rules! config_error {
    ($error:expr, $context:expr) => {
        $crate::error::ErrorHandler::config_error($error, $context)
    };
}