//! User models
//!
//! This module contains data structures for user information and preferences.

use serde::{Deserialize, Serialize};

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// Server ID this session belongs to
    pub server_id: String,
    /// User ID
    pub user_id: String,
    /// Username
    pub username: String,
    /// Access token for API requests
    pub access_token: String,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
    /// Session expiry timestamp
    pub expires_at: Option<i64>,
}

impl UserSession {
    /// Create a new user session
    pub fn new(
        server_id: String,
        user_id: String,
        username: String,
        access_token: String,
    ) -> Self {
        Self {
            server_id,
            user_id,
            username,
            access_token,
            refresh_token: None,
            expires_at: None,
        }
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            now >= expires_at
        } else {
            false
        }
    }
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub id: String,
    /// Username
    pub name: String,
    /// Display name
    pub display_name: Option<String>,
    /// User avatar image URL
    pub avatar_url: Option<String>,
    /// Whether user is administrator
    pub is_admin: bool,
    /// User preferences
    pub preferences: UserPreferences,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred language
    pub language: String,
    /// Preferred video quality
    pub video_quality: String,
    /// Audio language preference
    pub audio_language: Option<String>,
    /// Subtitle language preference
    pub subtitle_language: Option<String>,
}