//! Authentication and session management module
//!
//! This module handles user authentication, token management,
//! and session persistence across multiple servers.

pub mod session;
pub mod token;

pub use session::*;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication manager trait
#[async_trait]
pub trait AuthManager {
    /// Authenticate a user with a server
    async fn authenticate(&mut self, server_id: &str, username: &str, password: &str) -> Result<UserSession>;

    /// Get current user session for a server
    fn get_session(&self, server_id: &str) -> Option<&UserSession>;

    /// Get all active sessions
    fn get_all_sessions(&self) -> &HashMap<String, UserSession>;

    /// Remove a session
    fn remove_session(&mut self, server_id: &str) -> Option<UserSession>;

    /// Check if a session is valid (not expired)
    fn is_session_valid(&self, server_id: &str) -> bool;

    /// Refresh a session token if needed
    async fn refresh_session(&mut self, server_id: &str) -> Result<()>;
}

/// Server configuration for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub users: Vec<UserConfig>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

impl ServerConfig {
    pub fn new(id: String, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            users: Vec::new(),
            last_used: None,
        }
    }

    pub fn add_user(&mut self, user: UserConfig) {
        // Remove existing user with same id if present
        self.users.retain(|u| u.id != user.id);
        self.users.push(user);
    }

    pub fn get_user(&self, user_id: &str) -> Option<&UserConfig> {
        self.users.iter().find(|u| u.id == user_id)
    }

    pub fn remove_user(&mut self, user_id: &str) -> Option<UserConfig> {
        if let Some(pos) = self.users.iter().position(|u| u.id == user_id) {
            Some(self.users.remove(pos))
        } else {
            None
        }
    }

    pub fn update_last_used(&mut self) {
        self.last_used = Some(chrono::Utc::now());
    }
}

/// User configuration for a specific server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub id: String,
    pub name: String,
    pub server_id: String,
    pub remember_login: bool,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserConfig {
    pub fn new(id: String, name: String, server_id: String) -> Self {
        Self {
            id,
            name,
            server_id,
            remember_login: false,
            last_login: None,
        }
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(chrono::Utc::now());
    }
}