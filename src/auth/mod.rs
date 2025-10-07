use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Server configuration for connecting to Jellyfin instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub users: Vec<UserConfig>,
}

impl ServerConfig {
    pub fn new(id: String, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: UserConfig) {
        self.users.push(user);
    }
}

/// User configuration for a specific server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub id: String,
    pub name: String,
    pub remember_login: bool,
}

impl UserConfig {
    pub fn new(id: String, name: String, remember_login: bool) -> Self {
        Self {
            id,
            name,
            remember_login,
        }
    }
}

/// Active user session with authentication tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub server_id: String,
    pub user_id: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

impl UserSession {
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

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            now >= expires_at
        } else {
            false
        }
    }
}

/// Authentication credentials for login
#[derive(Debug, Clone, Serialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

/// Authentication response from Jellyfin server
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "User")]
    pub user: AuthUser,
}

/// User information from authentication response
#[derive(Debug, Deserialize)]
pub struct AuthUser {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Token manager for handling authentication tokens
#[derive(Debug, Default)]
pub struct TokenManager {
    sessions: HashMap<String, UserSession>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn store_session(&mut self, session: UserSession) {
        let key = format!("{}:{}", session.server_id, session.user_id);
        self.sessions.insert(key, session);
    }

    pub fn get_session(&self, server_id: &str, user_id: &str) -> Option<&UserSession> {
        let key = format!("{}:{}", server_id, user_id);
        self.sessions.get(&key)
    }

    pub fn remove_session(&mut self, server_id: &str, user_id: &str) {
        let key = format!("{}:{}", server_id, user_id);
        self.sessions.remove(&key);
    }

    pub fn clear_expired_sessions(&mut self) {
        self.sessions.retain(|_, session| !session.is_expired());
    }
}

/// Trait for authentication operations
pub trait AuthManager {
    /// Authenticate user with credentials
    async fn login(&mut self, server_url: &str, credentials: LoginCredentials) -> Result<UserSession>;

    /// Refresh an existing session token
    async fn refresh_token(&mut self, session: &UserSession) -> Result<UserSession>;

    /// Logout and invalidate session
    async fn logout(&mut self, session: &UserSession) -> Result<()>;

    /// Validate if a session is still active
    async fn validate_session(&self, session: &UserSession) -> Result<bool>;
}