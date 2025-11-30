//! Configuration models
//!
//! This module contains application configuration data structures.

use gpui::Global;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use super::{ServerConfig, UserSession};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// List of configured servers
    pub servers: Vec<ServerConfig>,
    /// ID of the currently active server
    pub active_server_id: Option<String>,
    /// User sessions keyed by server ID
    #[serde(default)]
    pub sessions: HashMap<String, UserSession>,
    /// UI-related settings
    pub ui_settings: UiSettings,
    /// Application preferences
    pub preferences: AppPreferences,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            active_server_id: None,
            sessions: HashMap::new(),
            ui_settings: UiSettings::default(),
            preferences: AppPreferences::default(),
        }
    }
}

/// UI-related settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    /// Theme preference (light, dark, auto)
    pub theme: String,
    /// Window size and position
    pub window_state: WindowState,
    /// UI scale factor
    pub scale_factor: f32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            window_state: WindowState::default(),
            scale_factor: 1.0,
        }
    }
}

/// Window state configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Window x position
    pub x: Option<f32>,
    /// Window y position
    pub y: Option<f32>,
    /// Whether window is maximized
    pub maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
            x: None,
            y: None,
            maximized: false,
        }
    }
}

/// Application preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    /// Auto-connect to last used server
    pub auto_connect: bool,
    /// Remember login credentials
    pub remember_credentials: bool,
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            auto_connect: true,
            remember_credentials: false,
            hardware_acceleration: true,
        }
    }
}

/// Global application state
pub struct AppState {
    /// Application configuration
    pub config: Arc<AppConfig>,
    /// Current connection status
    pub connection_status: ConnectionStatus,
    /// Loading state
    pub is_loading: bool,
}

impl AppState {
    /// Create new application state
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            connection_status: ConnectionStatus::Disconnected,
            is_loading: false,
        }
    }
}

impl Global for AppState {}

/// Connection status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Not connected to any server
    Disconnected,
    /// Attempting to connect
    Connecting,
    /// Successfully connected
    Connected,
    /// Connection failed
    Failed(String),
}