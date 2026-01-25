use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavedUser {
    pub username: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub saved_users: Vec<SavedUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub servers: Vec<Server>,
    pub active_server_id: Option<String>,
    pub last_connected_user_id: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;

        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        serde_json::from_str(&content).context("Failed to parse config file")
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(config_path, content).context("Failed to write config file")
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?
            .join("crabfin");
        Ok(config_dir.join("config.json"))
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn get_active_server(&self) -> Option<&Server> {
        self.active_server_id.as_ref().and_then(|id| {
            self.servers.iter().find(|s| &s.id == id)
        })
    }

    pub fn set_active_server(&mut self, server_id: String) {
        self.active_server_id = Some(server_id);
    }

    pub fn set_last_connected_user(&mut self, user_id: String) {
        self.last_connected_user_id = Some(user_id);
    }

    pub fn clear_active_server(&mut self) {
        self.active_server_id = None;
        self.last_connected_user_id = None;
    }
    
    pub fn clear_last_connected_user(&mut self) {
        self.last_connected_user_id = None;
    }

    pub fn add_saved_user(&mut self, server_id: &str, username: String, user_id: String) {
        if let Some(server) = self.servers.iter_mut().find(|s| s.id == server_id) {
            if !server.saved_users.iter().any(|u| u.user_id == user_id) {
                server.saved_users.push(SavedUser {
                    username,
                    user_id,
                });
            }
        }
    }

    pub fn remove_saved_user(&mut self, server_id: &str, user_id: &str) {
        if let Some(server) = self.servers.iter_mut().find(|s| s.id == server_id) {
            server.saved_users.retain(|u| u.user_id != user_id);
        }
    }

    pub fn get_saved_users(&self, server_id: &str) -> Vec<SavedUser> {
        self.servers
            .iter()
            .find(|s| s.id == server_id)
            .map(|s| s.saved_users.clone())
            .unwrap_or_default()
    }
}
