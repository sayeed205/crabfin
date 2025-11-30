//! Settings service
//!
//! This module handles application settings and configuration management.

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::models::AppConfig;

const APP_NAME: &str = "crabfin";
const CONFIG_FILE: &str = "config.json";

/// Service for managing application settings
pub struct SettingsService {
    /// Application configuration
    config: Arc<RwLock<AppConfig>>,
    /// Path to the configuration file
    config_path: PathBuf,
}

impl SettingsService {
    /// Create a new settings service
    pub fn new(default_config: Arc<AppConfig>) -> Result<Self> {
        tracing::debug!("Initializing settings service");

        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join(APP_NAME);

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_path = config_dir.join(CONFIG_FILE);

        Ok(Self {
            config: Arc::new(RwLock::new((*default_config).clone())),
            config_path,
        })
    }

    /// Load settings from storage
    pub async fn load_settings(&self) -> Result<()> {
        tracing::debug!("Loading application settings from {:?}", self.config_path);

        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)?;
            let config: AppConfig = serde_json::from_str(&content)?;

            let mut current_config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            *current_config = config;

            tracing::info!("Settings loaded successfully");
        } else {
            tracing::info!("No settings file found, using defaults");
            self.save_settings().await?;
        }

        Ok(())
    }

    /// Save settings to storage
    pub async fn save_settings(&self) -> Result<()> {
        tracing::debug!("Saving application settings to {:?}", self.config_path);

        let config = self.config.read().map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let content = serde_json::to_string_pretty(&*config)?;

        fs::write(&self.config_path, content)?;
        tracing::info!("Settings saved successfully");

        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> Arc<AppConfig> {
        let config = self.config.read().unwrap();
        Arc::new(config.clone())
    }

    /// Update configuration
    pub async fn update_config(&self, new_config: AppConfig) -> Result<()> {
        tracing::info!("Updating application configuration");

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            *config = new_config;
        }

        self.save_settings().await
    }

    /// Add a server to the configuration
    pub async fn add_server(&self, server: crate::models::ServerConfig) -> Result<()> {
        tracing::info!("Adding server: {}", server.name);

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            // Check if server already exists
            if !config.servers.iter().any(|s| s.id == server.id) {
                config.servers.push(server);
            } else {
                return Err(anyhow::anyhow!("Server already exists"));
            }
        }

        self.save_settings().await
    }

    /// Remove a server from the configuration
    pub async fn remove_server(&self, server_id: &str) -> Result<()> {
        tracing::info!("Removing server: {}", server_id);

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            config.servers.retain(|s| s.id != server_id);

            // Clear active server if it was the removed one
            if config.active_server_id.as_deref() == Some(server_id) {
                config.active_server_id = None;
            }
        }

        self.save_settings().await
    }

    /// Set the active server
    pub async fn set_active_server(&self, server_id: String) -> Result<()> {
        tracing::info!("Setting active server: {}", server_id);

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;

            // Verify server exists
            if !config.servers.iter().any(|s| s.id == server_id) {
                return Err(anyhow::anyhow!("Server not found"));
            }

            config.active_server_id = Some(server_id);
        }

        self.save_settings().await
    }

    /// Get the active server configuration
    pub fn get_active_server(&self) -> Option<crate::models::ServerConfig> {
        let config = self.config.read().unwrap();

        if let Some(active_id) = &config.active_server_id {
            config.servers.iter().find(|s| &s.id == active_id).cloned()
        } else {
            None
        }
    }

    /// Save a user session for a server
    pub async fn save_session(&self, server_id: String, session: crate::models::UserSession) -> Result<()> {
        tracing::info!("Saving session for server: {}", server_id);

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            config.sessions.insert(server_id, session);
        }

        self.save_settings().await
    }

    /// Get session for a specific server
    pub fn get_session(&self, server_id: &str) -> Option<crate::models::UserSession> {
        let config = self.config.read().unwrap();
        config.sessions.get(server_id).cloned()
    }

    /// Get session for the currently active server
    pub fn get_active_session(&self) -> Option<crate::models::UserSession> {
        let config = self.config.read().unwrap();

        if let Some(active_id) = &config.active_server_id {
            config.sessions.get(active_id).cloned()
        } else {
            None
        }
    }

    /// Clear session for a specific server (logout)
    pub async fn clear_session(&self, server_id: &str) -> Result<()> {
        tracing::info!("Clearing session for server: {}", server_id);

        {
            let mut config = self.config.write().map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
            config.sessions.remove(server_id);
        }

        self.save_settings().await
    }
}