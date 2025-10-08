//! User session management
//!
//! This module handles user sessions and authentication state.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use super::{AuthManager, ServerConfig, UserConfig};
use crate::client::{JellyfinApiClient, JellyfinClient};

/// User session containing authentication information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    pub server_id: String,
    pub user_id: String,
    pub username: String,
    pub access_token: String,
    pub server_name: String,
    pub server_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

impl UserSession {
    pub fn new(
        server_id: String,
        user_id: String,
        username: String,
        access_token: String,
        server_name: String,
        server_url: String,
    ) -> Self {
        Self {
            server_id,
            user_id,
            username,
            access_token,
            server_name,
            server_url,
            created_at: chrono::Utc::now(),
            expires_at: None, // Jellyfin tokens typically don't expire
            is_active: true,
        }
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false // No expiration set
        }
    }

    /// Check if the session is valid (active and not expired)
    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }

    /// Deactivate the session
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Update the access token
    pub fn update_token(&mut self, new_token: String) {
        self.access_token = new_token;
    }
}

/// Session manager implementation
#[derive(Debug)]
pub struct SessionManager {
    sessions: HashMap<String, UserSession>,
    servers: HashMap<String, ServerConfig>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            servers: HashMap::new(),
        }
    }

    /// Add a server configuration
    pub fn add_server(&mut self, server: ServerConfig) {
        debug!("Adding server configuration: {} ({})", server.name, server.id);
        self.servers.insert(server.id.clone(), server);
    }

    /// Get server configuration
    pub fn get_server(&self, server_id: &str) -> Option<&ServerConfig> {
        self.servers.get(server_id)
    }

    /// Get all server configurations
    pub fn get_servers(&self) -> &HashMap<String, ServerConfig> {
        &self.servers
    }

    /// Remove a server configuration
    pub fn remove_server(&mut self, server_id: &str) -> Option<ServerConfig> {
        // Also remove any active sessions for this server
        self.sessions.remove(server_id);
        self.servers.remove(server_id)
    }

    /// Update server last used timestamp
    pub fn update_server_last_used(&mut self, server_id: &str) {
        if let Some(server) = self.servers.get_mut(server_id) {
            server.update_last_used();
        }
    }
}

#[async_trait::async_trait]
impl AuthManager for SessionManager {
    async fn authenticate(&mut self, server_id: &str, username: &str, password: &str) -> Result<UserSession> {
        debug!("Authenticating user '{}' with server '{}'", username, server_id);

        let server = self.get_server(server_id)
            .ok_or_else(|| anyhow::anyhow!("Server configuration not found: {}", server_id))?;

        // Create API client for this server
        let client = JellyfinClient::new(server.url.clone())?;

        // Authenticate with the server
        let auth_result = client.authenticate(username, password).await?;

        // Create user session
        let session = UserSession::new(
            server_id.to_string(),
            auth_result.user.id.clone(),
            auth_result.user.name.clone(),
            auth_result.access_token,
            server.name.clone(),
            server.url.clone(),
        );

        // Store the session
        self.sessions.insert(server_id.to_string(), session.clone());

        // Update server last used
        self.update_server_last_used(server_id);

        // Update user configuration
        if let Some(server_config) = self.servers.get_mut(server_id) {
            let mut user_config = UserConfig::new(
                auth_result.user.id,
                auth_result.user.name,
                server_id.to_string(),
            );
            user_config.update_last_login();
            server_config.add_user(user_config);
        }

        info!("Authentication successful for user '{}' on server '{}'", username, server_id);
        Ok(session)
    }

    fn get_session(&self, server_id: &str) -> Option<&UserSession> {
        self.sessions.get(server_id)
    }

    fn get_all_sessions(&self) -> &HashMap<String, UserSession> {
        &self.sessions
    }

    fn remove_session(&mut self, server_id: &str) -> Option<UserSession> {
        debug!("Removing session for server: {}", server_id);
        self.sessions.remove(server_id)
    }

    fn is_session_valid(&self, server_id: &str) -> bool {
        self.sessions
            .get(server_id)
            .map(|session| session.is_valid())
            .unwrap_or(false)
    }

    async fn refresh_session(&mut self, server_id: &str) -> Result<()> {
        debug!("Refreshing session for server: {}", server_id);

        // For now, Jellyfin tokens typically don't need refreshing
        // This is a placeholder for future implementation if needed
        warn!("Session refresh not implemented yet for server: {}", server_id);

        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}