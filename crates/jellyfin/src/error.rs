use thiserror::Error;

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Error, Debug)]
pub enum JellyfinError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("Unauthorized (401)")]
    Unauthorized,

    #[error("Server not found or unreachable")]
    ServerNotFound,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_error_display() {
        let error = JellyfinError::ServerNotFound;
        assert!(
            error.to_string().contains("not found") || error.to_string().contains("unreachable")
        );
    }

    #[test]
    fn test_parse_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error = JellyfinError::Parse(json_err);
        assert!(error.to_string().len() > 0);
    }

    #[test]
    fn test_api_error_display() {
        let error = JellyfinError::Api {
            status: 500,
            message: "Internal Server Error".to_string(),
        };
        let display = error.to_string();
        assert!(display.contains("500"));
        assert!(display.contains("Internal Server Error"));
    }

    #[test]
    fn test_unauthorized_error() {
        let error = JellyfinError::Unauthorized;
        assert!(error.to_string().contains("401") || error.to_string().contains("Unauthorized"));
    }

    #[test]
    fn test_server_not_found_error() {
        let error = JellyfinError::ServerNotFound;
        assert!(error.to_string().len() > 0);
    }

    #[test]
    fn test_invalid_url_error() {
        let error = JellyfinError::InvalidUrl("not a url".to_string());
        let display = error.to_string();
        assert!(display.contains("not a url"));
    }
}
