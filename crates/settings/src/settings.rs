use anyhow::{Context, Result};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub device_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    pub id: String,
    pub server_id: Uuid,
    pub username: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppSettings {
    pub last_server_id: Option<Uuid>,
    pub last_user_id: Option<String>,
    pub default_server_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub settings: AppSettings,
    #[serde(default)]
    pub servers: Vec<ServerConfig>,
    #[serde(default)]
    pub users: Vec<UserConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            settings: AppSettings::default(),
            servers: Vec::new(),
            users: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ConfigVersions {
    V1(Config),
}

impl ConfigVersions {
    pub fn parse(json: &str) -> serde_json::Result<Config> {
        let version: ConfigVersions = serde_json::from_str(json)?;
        Ok(version.into())
    }
}

impl From<ConfigVersions> for Config {
    fn from(version: ConfigVersions) -> Self {
        match version {
            ConfigVersions::V1(config) => config,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = get_config_path().context("Could not determine config path")?;
        Self::load_from(&path)
    }

    pub fn save(&self) -> Result<()> {
        let path = get_config_path().context("Could not determine config path")?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        self.save_to(&path)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let config = ConfigVersions::parse(&content)?;
        Ok(config)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_server(&mut self, server: ServerConfig) {
        if let Some(index) = self.servers.iter().position(|s| s.id == server.id) {
            self.servers[index] = server;
        } else {
            self.servers.push(server);
        }
    }

    pub fn remove_server(&mut self, id: Uuid) {
        self.servers.retain(|s| s.id != id);
        // Also remove associated users
        self.users.retain(|u| u.server_id != id);
        // Clear last_server_id if it matches
        if self.settings.last_server_id == Some(id) {
            self.settings.last_server_id = None;
        }
        if self.settings.default_server_id == Some(id) {
            self.settings.default_server_id = None;
        }
    }

    pub fn add_user(&mut self, user: UserConfig) {
        if let Some(index) = self
            .users
            .iter()
            .position(|u| u.id == user.id && u.server_id == user.server_id)
        {
            self.users[index] = user;
        } else {
            self.users.push(user);
        }
    }

    pub fn get_server(&self, id: Uuid) -> Option<&ServerConfig> {
        self.servers.iter().find(|s| s.id == id)
    }

    pub fn get_user(&self, id: &str, server_id: Uuid) -> Option<&UserConfig> {
        self.users
            .iter()
            .find(|u| u.id == id && u.server_id == server_id)
    }

    pub fn get_default_or_first_server(&self) -> Option<&ServerConfig> {
        if let Some(id) = self.settings.default_server_id {
            if let Some(server) = self.get_server(id) {
                return Some(server);
            }
        }
        self.servers.first()
    }

    pub fn get_users_for_server(&self, server_id: Uuid) -> Vec<&UserConfig> {
        self.users
            .iter()
            .filter(|u| u.server_id == server_id)
            .collect()
    }

    pub fn get_remember_me_user_for_server(&self, server_id: Uuid) -> Option<&UserConfig> {
        self.users
            .iter()
            .find(|u| u.server_id == server_id && u.remember_me)
    }

    pub fn reload(&mut self) -> Result<()> {
        let path = get_config_path().context("Could not determine config path")?;
        let new_config = Self::load_from(&path)?;
        *self = new_config;
        Ok(())
    }
}

pub struct SettingsWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
}

impl SettingsWatcher {
    pub fn new(path: &Path) -> Result<Self> {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx)?;

        if let Some(parent) = path.parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;
        }

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    pub fn poll(&self) -> bool {
        let mut changed = false;
        while let Ok(res) = self.rx.try_recv() {
            if let Ok(event) = res {
                match event.kind {
                    notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                        changed = true;
                    }
                    _ => {}
                }
            }
        }
        changed
    }
}

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("crabfin"))
}

pub fn get_config_path() -> Option<PathBuf> {
    get_config_dir().map(|d| d.join("config.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_serialization() {
        let id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        let server = ServerConfig {
            id,
            name: "Test Server".to_string(),
            url: "http://localhost:8096".to_string(),
            device_id,
        };

        let json = serde_json::to_string(&server).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(server, deserialized);
        assert_eq!(server.id, id);
        assert_eq!(server.device_id, device_id);
    }

    #[test]
    fn test_user_config_serialization() {
        let server_id = Uuid::new_v4();
        let user = UserConfig {
            id: "user123".to_string(),
            server_id,
            username: "testuser".to_string(),
            remember_me: true,
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: UserConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(user, deserialized);
        assert!(deserialized.remember_me);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, 1);
        assert!(config.servers.is_empty());
        assert!(config.users.is_empty());
        assert!(config.settings.last_server_id.is_none());
        assert!(config.settings.default_server_id.is_none());
    }

    #[test]
    fn test_full_config_serialization() {
        let mut config = Config::default();
        let server_id = Uuid::new_v4();

        config.servers.push(ServerConfig {
            id: server_id,
            name: "Local".to_string(),
            url: "http://localhost".to_string(),
            device_id: Uuid::new_v4(),
        });

        config.users.push(UserConfig {
            id: "u1".to_string(),
            server_id,
            username: "me".to_string(),
            remember_me: true,
        });

        config.settings.last_server_id = Some(server_id);

        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
        assert_eq!(deserialized.servers.len(), 1);
        assert_eq!(deserialized.users.len(), 1);
    }

    #[test]
    fn test_config_versioning() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();

        let parsed = ConfigVersions::parse(&json).unwrap();
        assert_eq!(parsed.version, 1);
    }

    #[test]
    fn test_load_save_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let mut config = Config::default();
        config.servers.push(ServerConfig {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
            url: "http://test".to_string(),
            device_id: Uuid::new_v4(),
        });

        config.save_to(&path).unwrap();
        assert!(path.exists());

        let loaded = Config::load_from(&path).unwrap();
        assert_eq!(config, loaded);
        assert_eq!(loaded.servers.len(), 1);
    }

    #[test]
    fn test_load_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let config = Config::load_from(&path).unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_server_management() {
        let mut config = Config::default();
        let id = Uuid::new_v4();
        let server = ServerConfig {
            id,
            name: "S1".to_string(),
            url: "u1".to_string(),
            device_id: Uuid::new_v4(),
        };

        config.add_server(server.clone());
        assert_eq!(config.servers.len(), 1);
        assert_eq!(config.get_server(id), Some(&server));

        // Update
        let mut updated = server.clone();
        updated.name = "S2".to_string();
        config.add_server(updated.clone());
        assert_eq!(config.servers.len(), 1);
        assert_eq!(config.get_server(id).unwrap().name, "S2");

        config.remove_server(id);
        assert!(config.servers.is_empty());
    }

    #[test]
    fn test_hot_reload() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let mut config = Config::default();
        config.save_to(&path).unwrap();

        let watcher = SettingsWatcher::new(&path).unwrap();

        config.settings.last_user_id = Some("test".to_string());
        config.save_to(&path).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(500));

        assert!(watcher.poll());

        let fresh = Config::load_from(&path).unwrap();
        assert_eq!(fresh.settings.last_user_id, Some("test".to_string()));
    }

    #[test]
    fn test_config_helpers() {
        let mut config = Config::default();
        let s1_id = Uuid::new_v4();
        let s2_id = Uuid::new_v4();

        let s1 = ServerConfig {
            id: s1_id,
            name: "S1".to_string(),
            url: "u1".to_string(),
            device_id: Uuid::new_v4(),
        };
        let s2 = ServerConfig {
            id: s2_id,
            name: "S2".to_string(),
            url: "u2".to_string(),
            device_id: Uuid::new_v4(),
        };
        config.add_server(s1.clone());
        config.add_server(s2.clone());

        let u1 = UserConfig {
            id: "u1".to_string(),
            server_id: s1_id,
            username: "user1".to_string(),
            remember_me: false,
        };
        let u2 = UserConfig {
            id: "u2".to_string(),
            server_id: s1_id,
            username: "user2".to_string(),
            remember_me: true,
        };
        let u3 = UserConfig {
            id: "u3".to_string(),
            server_id: s2_id,
            username: "user3".to_string(),
            remember_me: false,
        };
        config.add_user(u1.clone());
        config.add_user(u2.clone());
        config.add_user(u3.clone());

        assert_eq!(config.get_default_or_first_server(), Some(&s1));

        config.settings.default_server_id = Some(s2_id);
        assert_eq!(config.get_default_or_first_server(), Some(&s2));

        config.settings.default_server_id = Some(Uuid::new_v4());
        assert_eq!(config.get_default_or_first_server(), Some(&s1));

        let users_s1 = config.get_users_for_server(s1_id);
        assert_eq!(users_s1.len(), 2);
        assert!(users_s1.contains(&&u1));
        assert!(users_s1.contains(&&u2));

        let users_s2 = config.get_users_for_server(s2_id);
        assert_eq!(users_s2.len(), 1);
        assert!(users_s2.contains(&&u3));

        assert_eq!(config.get_remember_me_user_for_server(s1_id), Some(&u2));
        assert_eq!(config.get_remember_me_user_for_server(s2_id), None);
    }
}
