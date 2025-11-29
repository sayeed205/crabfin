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
}