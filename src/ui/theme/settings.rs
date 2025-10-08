use crate::ui::theme::Color;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Theme settings for persistence across app sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    /// Whether dynamic theming based on wallpaper is enabled
    pub dynamic_enabled: bool,

    /// Custom color selected by the user (when not using dynamic theming)
    pub custom_color: Option<Color>,

    /// Path to the last processed wallpaper (for change detection)
    pub last_wallpaper_path: Option<PathBuf>,

    /// Whether to animate theme transitions
    pub transition_animations: bool,

    /// Whether to follow system dark/light mode preference
    pub follow_system_theme: bool,

    /// Duration of theme transition animations in milliseconds
    pub transition_duration_ms: u64,
}

impl ThemeSettings {
    /// Create new theme settings with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create theme settings with dynamic theming enabled
    pub fn with_dynamic_enabled() -> Self {
        Self {
            dynamic_enabled: true,
            ..Default::default()
        }
    }

    /// Create theme settings with a custom color
    pub fn with_custom_color(color: Color) -> Self {
        Self {
            dynamic_enabled: false,
            custom_color: Some(color),
            ..Default::default()
        }
    }

    /// Enable or disable dynamic theming
    pub fn set_dynamic_enabled(&mut self, enabled: bool) {
        self.dynamic_enabled = enabled;
        if enabled {
            // When enabling dynamic mode, clear custom color
            self.custom_color = None;
        }
    }

    /// Set a custom color and disable dynamic theming
    pub fn set_custom_color(&mut self, color: Color) {
        self.custom_color = Some(color);
        self.dynamic_enabled = false;
    }

    /// Clear custom color setting
    pub fn clear_custom_color(&mut self) {
        self.custom_color = None;
    }

    /// Update the last wallpaper path
    pub fn update_wallpaper_path(&mut self, path: Option<PathBuf>) {
        self.last_wallpaper_path = path;
    }

    /// Check if wallpaper has changed since last update
    pub fn has_wallpaper_changed(&self, current_path: &Option<PathBuf>) -> bool {
        self.last_wallpaper_path != *current_path
    }

    /// Enable or disable transition animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.transition_animations = enabled;
    }

    /// Set transition duration in milliseconds
    pub fn set_transition_duration(&mut self, duration_ms: u64) {
        self.transition_duration_ms = duration_ms;
    }

    /// Get transition duration as std::time::Duration
    pub fn get_transition_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.transition_duration_ms)
    }

    /// Enable or disable following system theme preference
    pub fn set_follow_system_theme(&mut self, follow: bool) {
        self.follow_system_theme = follow;
    }

    /// Reset to default settings
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Validate settings and fix any invalid values
    pub fn validate_and_fix(&mut self) {
        // Ensure transition duration is within reasonable bounds
        if self.transition_duration_ms < 50 {
            self.transition_duration_ms = 50;
        } else if self.transition_duration_ms > 2000 {
            self.transition_duration_ms = 2000;
        }

        // If both dynamic and custom color are set, prefer dynamic
        if self.dynamic_enabled && self.custom_color.is_some() {
            self.custom_color = None;
        }
    }

    /// Check if the settings represent a valid configuration
    pub fn is_valid(&self) -> bool {
        // Must have either dynamic enabled or a custom color
        self.dynamic_enabled || self.custom_color.is_some()
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            dynamic_enabled: true,
            custom_color: None,
            last_wallpaper_path: None,
            transition_animations: true,
            follow_system_theme: true,
            transition_duration_ms: 300,
        }
    }
}

/// Theme mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    /// Resolve the theme mode to a boolean (true = dark, false = light)
    pub fn resolve(&self, system_is_dark: bool) -> bool {
        match self {
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
            ThemeMode::System => system_is_dark,
        }
    }
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::System
    }
}

/// Theme source enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeSource {
    /// Use colors extracted from system wallpaper
    Wallpaper,
    /// Use a custom user-selected color
    Custom(Color),
    /// Use default Material 3 colors
    Default,
}

impl ThemeSource {
    /// Get the source color if available
    pub fn get_source_color(&self) -> Option<Color> {
        match self {
            ThemeSource::Custom(color) => Some(*color),
            _ => None,
        }
    }

    /// Check if this source requires wallpaper monitoring
    pub fn requires_wallpaper_monitoring(&self) -> bool {
        matches!(self, ThemeSource::Wallpaper)
    }
}

impl Default for ThemeSource {
    fn default() -> Self {
        ThemeSource::Wallpaper
    }
}