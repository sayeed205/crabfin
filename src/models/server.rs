//! Server models
//!
//! This module contains data structures for Jellyfin server information.

use serde::{Deserialize, Serialize};

/// Jellyfin server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Unique server identifier
    pub id: String,
    /// Server display name
    pub name: String,
    /// Server URL
    pub url: String,
    /// List of users configured for this server
    pub users: Vec<UserConfig>,
    /// Whether this server is currently active
    pub is_active: bool,
}

impl ServerConfig {
    /// Create a new server configuration
    pub fn new(id: String, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            users: Vec::new(),
            is_active: false,
        }
    }
}

/// User configuration for a specific server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// User ID on the server
    pub user_id: String,
    /// Username
    pub username: String,
    /// Whether to remember this user
    pub remember: bool,
}