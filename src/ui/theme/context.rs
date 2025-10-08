use crate::ui::theme::{Color, MaterialPalette, ThemeAnimationManager, ThemeSettings, ThemeSource};
use gpui::*;
use std::time::Duration;

/// GPUI reactive context for theme state management
pub struct ThemeContext {
    /// Current Material 3 color palette
    pub current_palette: MaterialPalette,

    /// Whether the current theme is in dark mode
    pub is_dark_mode: bool,

    /// Current theme source (wallpaper, custom, or default)
    pub theme_source: ThemeSource,

    /// Theme settings
    pub settings: ThemeSettings,

    /// Duration for theme transition animations
    pub transition_duration: Duration,

    /// Whether system dark/light mode detection is active
    pub system_theme_detection: bool,

    /// Animation manager for smooth transitions
    pub animation_manager: ThemeAnimationManager,

    /// Animation timer task (not cloned, reset on clone)
    #[allow(dead_code)]
    pub animation_task: Option<Task<()>>,
}

impl ThemeContext {
    /// Create a new theme context with default Material 3 palette
    pub fn new() -> Self {
        Self {
            current_palette: MaterialPalette::default(),
            is_dark_mode: false,
            theme_source: ThemeSource::default(),
            settings: ThemeSettings::default(),
            transition_duration: Duration::from_millis(300),
            system_theme_detection: true,
            animation_manager: ThemeAnimationManager::new(),
            animation_task: None,
        }
    }

    /// Create theme context with specific palette
    pub fn with_palette(palette: MaterialPalette) -> Self {
        Self {
            current_palette: palette,
            is_dark_mode: false,
            theme_source: ThemeSource::default(),
            settings: ThemeSettings::default(),
            transition_duration: Duration::from_millis(300),
            system_theme_detection: true,
            animation_manager: ThemeAnimationManager::new(),
            animation_task: None,
        }
    }

    /// Update the current palette
    pub fn update_palette(&mut self, palette: MaterialPalette) {
        if self.current_palette.primary.light != palette.primary.light
            || self.current_palette.primary.dark != palette.primary.dark {
            self.current_palette = palette;
        }
    }

    /// Set dark mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.is_dark_mode = dark;
    }

    /// Toggle between light and dark mode
    pub fn toggle_dark_mode(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
    }

    /// Update theme source
    pub fn set_theme_source(&mut self, source: ThemeSource) {
        self.theme_source = source;
    }

    /// Update theme settings
    pub fn update_settings(&mut self, settings: ThemeSettings) {
        self.settings = settings;
        self.transition_duration = self.settings.get_transition_duration();
    }

    /// Enable or disable system theme detection
    pub fn set_system_theme_detection(&mut self, enabled: bool) {
        self.system_theme_detection = enabled;
    }

    /// Get the current palette
    pub fn get_palette(&self) -> &MaterialPalette {
        &self.current_palette
    }

    /// Get the current palette for rendering (includes animation state)
    pub fn get_render_palette(&self) -> MaterialPalette {
        self.get_current_palette()
    }

    /// Get current dark mode state
    pub fn is_dark_mode(&self) -> bool {
        self.is_dark_mode
    }

    /// Get current theme source
    pub fn get_theme_source(&self) -> &ThemeSource {
        &self.theme_source
    }

    /// Get theme settings
    pub fn get_settings(&self) -> &ThemeSettings {
        &self.settings
    }

    /// Get transition duration
    pub fn get_transition_duration(&self) -> Duration {
        self.transition_duration
    }

    /// Check if system theme detection is enabled
    pub fn is_system_theme_detection_enabled(&self) -> bool {
        self.system_theme_detection
    }

    /// Get surface colors for current theme mode
    pub fn get_surface_colors(&self) -> crate::ui::theme::SurfaceColorSet {
        let palette = self.get_current_palette();
        palette.get_surface_colors(self.is_dark_mode)
    }

    /// Get text colors for current theme mode
    pub fn get_text_colors(&self) -> crate::ui::theme::TextColors {
        let palette = self.get_current_palette();
        palette.get_text_colors(self.is_dark_mode)
    }

    /// Get accent colors for current theme mode
    pub fn get_accent_colors(&self) -> crate::ui::theme::AccentColors {
        let palette = self.get_current_palette();
        palette.get_accent_colors(self.is_dark_mode)
    }

    /// Apply theme transition with animation
    pub fn apply_theme_transition(&mut self, new_palette: MaterialPalette) {
        if !self.settings.transition_animations {
            // No animation, apply immediately
            self.update_palette(new_palette);
            return;
        }

        // Start animation from current palette to new palette
        let old_palette = self.current_palette.clone();
        self.animation_manager.start_transition(old_palette, new_palette.clone(), Some(self.transition_duration));

        // Update the target palette
        self.current_palette = new_palette;

        // Note: In a full implementation, we would set up proper animation timers
        // For now, we just apply the new palette immediately
    }

    /// Get the current palette (animated if transition is active)
    pub fn get_current_palette(&self) -> MaterialPalette {
        if let Some(animated_palette) = self.animation_manager.current_palette() {
            animated_palette
        } else {
            self.current_palette.clone()
        }
    }

    /// Check if animation is currently active
    pub fn is_animating(&self) -> bool {
        self.animation_manager.is_animating()
    }

    /// Handle system theme change
    pub fn handle_system_theme_change(&mut self, is_dark: bool) {
        if self.system_theme_detection && self.settings.follow_system_theme {
            self.set_dark_mode(is_dark);
        }
    }

    /// Check if dynamic theming is enabled
    pub fn is_dynamic_theming_enabled(&self) -> bool {
        self.settings.dynamic_enabled
    }

    /// Get custom color if set
    pub fn get_custom_color(&self) -> Option<Color> {
        self.settings.custom_color
    }

    /// Set custom color and disable dynamic theming
    pub fn set_custom_color(&mut self, color: Color) {
        self.settings.set_custom_color(color);
        self.theme_source = ThemeSource::Custom(color);
    }

    /// Enable dynamic theming
    pub fn enable_dynamic_theming(&mut self) {
        self.settings.set_dynamic_enabled(true);
        self.theme_source = ThemeSource::Wallpaper;
    }

    /// Disable dynamic theming
    pub fn disable_dynamic_theming(&mut self) {
        self.settings.set_dynamic_enabled(false);
        if self.settings.custom_color.is_none() {
            self.theme_source = ThemeSource::Default;
        }
    }
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ThemeContext {
    fn clone(&self) -> Self {
        Self {
            current_palette: self.current_palette.clone(),
            is_dark_mode: self.is_dark_mode,
            theme_source: self.theme_source.clone(),
            settings: self.settings.clone(),
            transition_duration: self.transition_duration,
            system_theme_detection: self.system_theme_detection,
            animation_manager: self.animation_manager.clone(),
            animation_task: None, // Don't clone the task
        }
    }
}

/// Make ThemeContext a global type
impl Global for ThemeContext {}

/// Global theme access functions for components
impl ThemeContext {
    /// Get the global theme context from GPUI app context
    pub fn global(cx: &App) -> &ThemeContext {
        cx.global::<ThemeContext>()
    }

    /// Update the global theme context
    pub fn update_global<F, R>(cx: &mut App, f: F) -> R
    where
        F: FnOnce(&mut ThemeContext) -> R,
    {
        let mut theme = cx.global::<ThemeContext>().clone();
        let result = f(&mut theme);
        cx.set_global(theme);
        result
    }
}

/// System theme detection utilities
pub struct SystemThemeDetector;

impl SystemThemeDetector {
    /// Detect if the system is currently in dark mode
    pub fn is_system_dark_mode() -> bool {
        // Platform-specific system theme detection
        #[cfg(target_os = "macos")]
        {
            Self::is_macos_dark_mode()
        }

        #[cfg(target_os = "linux")]
        {
            Self::is_linux_dark_mode()
        }

        #[cfg(target_os = "windows")]
        {
            Self::is_windows_dark_mode()
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            false // Default to light mode on unsupported platforms
        }
    }

    /// Start monitoring system theme changes
    pub fn start_monitoring<F>(callback: F) -> anyhow::Result<()>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        // Platform-specific system theme monitoring
        #[cfg(target_os = "macos")]
        {
            Self::start_macos_monitoring(callback)
        }

        #[cfg(target_os = "linux")]
        {
            Self::start_linux_monitoring(callback)
        }

        #[cfg(target_os = "windows")]
        {
            Self::start_windows_monitoring(callback)
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            Ok(()) // No-op on unsupported platforms
        }
    }

    #[cfg(target_os = "macos")]
    fn is_macos_dark_mode() -> bool {
        // TODO: Implement macOS dark mode detection using NSUserDefaults
        // For now, return false as a placeholder
        false
    }

    #[cfg(target_os = "macos")]
    fn start_macos_monitoring<F>(_callback: F) -> anyhow::Result<()>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        // TODO: Implement macOS system theme monitoring using NSDistributedNotificationCenter
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn is_linux_dark_mode() -> bool {
        // Try to detect dark mode through various Linux desktop environment methods

        // GNOME/GTK
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            if let Ok(theme) = String::from_utf8(output.stdout) {
                return theme.to_lowercase().contains("dark");
            }
        }

        // KDE Plasma
        if let Ok(output) = std::process::Command::new("kreadconfig5")
            .args(&["--group", "General", "--key", "ColorScheme"])
            .output()
        {
            if let Ok(scheme) = String::from_utf8(output.stdout) {
                return scheme.to_lowercase().contains("dark");
            }
        }

        // Fallback: check environment variables
        if let Ok(theme) = std::env::var("GTK_THEME") {
            return theme.to_lowercase().contains("dark");
        }

        false
    }

    #[cfg(target_os = "linux")]
    fn start_linux_monitoring<F>(_callback: F) -> anyhow::Result<()>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        // TODO: Implement Linux system theme monitoring using dbus or file watchers
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn is_windows_dark_mode() -> bool {
        // TODO: Implement Windows dark mode detection using registry
        false
    }

    #[cfg(target_os = "windows")]
    fn start_windows_monitoring<F>(_callback: F) -> anyhow::Result<()>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        // TODO: Implement Windows system theme monitoring using registry watchers
        Ok(())
    }
}

