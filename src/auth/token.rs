//! Token management
//!
//! This module handles authentication tokens, refresh logic, and token storage.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, warn};

/// Token information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TokenInfo {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: None,
            refresh_token: None,
            scope: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    pub fn with_expires_in(mut self, expires_in: u64) -> Self {
        self.expires_in = Some(expires_in);
        self
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_in) = self.expires_in {
            let expires_at = self.created_at + chrono::Duration::seconds(expires_in as i64);
            chrono::Utc::now() > expires_at
        } else {
            false // No expiration set
        }
    }

    /// Get the expiration time
    pub fn expires_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.expires_in.map(|expires_in| {
            self.created_at + chrono::Duration::seconds(expires_in as i64)
        })
    }

    /// Check if the token needs refresh (expires within 5 minutes)
    pub fn needs_refresh(&self) -> bool {
        if let Some(expires_at) = self.expires_at() {
            let refresh_threshold = chrono::Utc::now() + chrono::Duration::minutes(5);
            expires_at < refresh_threshold
        } else {
            false
        }
    }
}

/// Token manager for handling multiple server tokens
#[derive(Debug)]
pub struct TokenManager {
    tokens: HashMap<String, TokenInfo>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Store a token for a server
    pub fn store_token(&mut self, server_id: String, token: TokenInfo) {
        debug!("Storing token for server: {}", server_id);
        self.tokens.insert(server_id, token);
    }

    /// Get a token for a server
    pub fn get_token(&self, server_id: &str) -> Option<&TokenInfo> {
        self.tokens.get(server_id)
    }

    /// Get a valid (non-expired) token for a server
    pub fn get_valid_token(&self, server_id: &str) -> Option<&TokenInfo> {
        self.tokens.get(server_id).and_then(|token| {
            if token.is_expired() {
                warn!("Token for server '{}' is expired", server_id);
                None
            } else {
                Some(token)
            }
        })
    }

    /// Remove a token for a server
    pub fn remove_token(&mut self, server_id: &str) -> Option<TokenInfo> {
        debug!("Removing token for server: {}", server_id);
        self.tokens.remove(server_id)
    }

    /// Check if a server has a valid token
    pub fn has_valid_token(&self, server_id: &str) -> bool {
        self.get_valid_token(server_id).is_some()
    }

    /// Get all servers that need token refresh
    pub fn get_servers_needing_refresh(&self) -> Vec<String> {
        self.tokens
            .iter()
            .filter_map(|(server_id, token)| {
                if token.needs_refresh() {
                    Some(server_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Update a token for a server
    pub fn update_token(&mut self, server_id: &str, new_access_token: String) -> Result<()> {
        if let Some(token) = self.tokens.get_mut(server_id) {
            debug!("Updating token for server: {}", server_id);
            token.access_token = new_access_token;
            token.created_at = chrono::Utc::now();
            Ok(())
        } else {
            error!("No token found for server: {}", server_id);
            Err(anyhow::anyhow!("No token found for server: {}", server_id))
        }
    }

    /// Clear all tokens
    pub fn clear_all(&mut self) {
        debug!("Clearing all tokens");
        self.tokens.clear();
    }

    /// Get all server IDs with tokens
    pub fn get_server_ids(&self) -> Vec<String> {
        self.tokens.keys().cloned().collect()
    }

    /// Get token count
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}