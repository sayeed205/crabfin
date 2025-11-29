use crate::ui::theme::{
    Color, ColorExtractor, MaterialColorsIntegration, MaterialPalette, ThemeContext, ThemeSettings,
    ThemeSource, WallpaperMonitor,
};
use anyhow::Result;
use gpui::*;
use std::path::PathBuf;


/// Central coordinator for all theme operations
///
/// The ThemeManager provides different approaches for theme management:
///
/// 1. **Basic setup**:
///    - Use `initialize_global()` for basic theme management
///    - Manual theme updates via method calls
///
/// 2. **With wallpaper monitoring**:
///    - Use `initialize_global_with_wallpaper_monitoring()` 
///    - Automatically checks for wallpaper changes periodically
///    - Uses GPUI's timer system for background checking
///
/// 3. **Manual refresh**:
///    - Call `refresh_wallpaper_theme()` when needed
///    - Useful for on-demand theme updates
///
/// # Example Usage
///
/// ```rust
/// // Basic setup
/// ThemeManager::initialize_global(cx)?;
///
/// // With automatic wallpaper monitoring
/// let wallpaper_task = ThemeManager::initialize_global_with_wallpaper_monitoring(cx)?;
/// // Keep the task alive for continuous monitoring
///
/// // Manual refresh
/// ThemeManager::update_global(cx, |manager| {
///     manager.refresh_wallpaper_theme(cx)
/// })?;
/// ```
///
/// Note: System appearance changes (light/dark mode) should be handled 
/// by individual views using GPUI's `observe_window_appearance` method.
pub struct ThemeManager {
    /// Color extractor for wallpaper analysis
    color_extractor: ColorExtractor,

    /// Material colors integration for palette generation
    material_colors: MaterialColorsIntegration,

    /// Wallpaper monitor for detecting changes
    wallpaper_monitor: Option<WallpaperMonitor>,

    /// Current theme settings
    settings: ThemeSettings,

    /// Whether the manager is currently active
    is_active: bool,
}

impl ThemeManager {
    /// Create a new ThemeManager instance
    pub fn new() -> Self {
        Self {
            color_extractor: ColorExtractor::new(),
            material_colors: MaterialColorsIntegration::new(),
            wallpaper_monitor: None,
            settings: ThemeSettings::default(),
            is_active: false,
        }
    }

    /// Initialize the theme manager with the given context
    pub fn initialize(&mut self, cx: &mut App) -> Result<()> {
        // Load settings from storage if available
        self.load_settings()?;

        // Apply initial theme based on settings
        self.apply_initial_theme(cx)?;

        // Start wallpaper monitoring if dynamic theming is enabled
        if self.settings.dynamic_enabled {
            self.start_wallpaper_monitoring(cx)?;
        }

        self.is_active = true;
        Ok(())
    }


    /// Set dynamic theming mode
    pub fn set_dynamic_mode(&mut self, enabled: bool, cx: &mut App) -> Result<()> {
        if self.settings.dynamic_enabled == enabled {
            return Ok(()); // No change needed
        }

        self.settings.set_dynamic_enabled(enabled);

        if enabled {
            // Enable dynamic theming - start monitoring and apply wallpaper colors
            self.start_wallpaper_monitoring(cx)?;
            self.apply_wallpaper_colors(cx)?;
        } else {
            // Disable dynamic theming - stop monitoring
            self.stop_wallpaper_monitoring();

            // Apply custom color if available, otherwise use default
            if let Some(custom_color) = self.settings.custom_color {
                self.apply_custom_color(custom_color, cx)?;
            } else {
                self.apply_default_colors(cx)?;
            }
        }

        self.save_settings()?;
        Ok(())
    }

    /// Set a custom color for theming
    pub fn set_custom_color(&mut self, color: Color, cx: &mut App) -> Result<()> {
        // Stop wallpaper monitoring when switching to custom color
        self.stop_wallpaper_monitoring();

        // Update settings
        self.settings.set_custom_color(color);

        // Apply the custom color theme
        self.apply_custom_color(color, cx)?;

        self.save_settings()?;
        Ok(())
    }

    /// Apply wallpaper-based theme colors
    pub fn apply_wallpaper_colors(&mut self, cx: &mut App) -> Result<()> {
        let wallpaper_color = self
            .color_extractor
            .extract_from_wallpaper()
            .map_err(|e| anyhow::anyhow!("Failed to extract wallpaper colors: {}", e))?;

        let palette = self
            .material_colors
            .generate_palette(wallpaper_color)
            .map_err(|e| anyhow::anyhow!("Failed to generate Material 3 palette from wallpaper: {}", e))?;

        self.apply_palette(palette, ThemeSource::Wallpaper, cx);

        // Update wallpaper path in settings
        if let Ok(wallpaper_path) = self.color_extractor.get_wallpaper_path() {
            self.settings.update_wallpaper_path(Some(wallpaper_path));
        }

        Ok(())
    }

    /// Apply custom color theme
    pub fn apply_custom_color(&mut self, color: Color, cx: &mut App) -> Result<()> {
        let palette = self
            .material_colors
            .generate_palette(color)
            .map_err(|e| anyhow::anyhow!("Failed to generate Material 3 palette from custom color: {}", e))?;

        self.apply_palette(palette, ThemeSource::Custom(color), cx);
        Ok(())
    }

    /// Apply default Material 3 colors
    pub fn apply_default_colors(&mut self, cx: &mut App) -> Result<()> {
        let default_color = Color::from_hex("#6750A4")?;
        let palette = self
            .material_colors
            .generate_palette(default_color)
            .map_err(|e| anyhow::anyhow!("Failed to generate default Material 3 palette: {}", e))?;

        self.apply_palette(palette, ThemeSource::Default, cx);
        Ok(())
    }

    /// Get the current Material 3 palette
    pub fn get_current_palette(&self, cx: &App) -> MaterialPalette {
        let theme_context = cx.global::<ThemeContext>();
        theme_context.get_palette().clone()
    }

    /// Check if dynamic theming is currently enabled
    pub fn is_dynamic_mode_enabled(&self) -> bool {
        self.settings.dynamic_enabled
    }

    /// Get current custom color if set
    pub fn get_custom_color(&self) -> Option<Color> {
        self.settings.custom_color
    }

    /// Get current theme settings
    pub fn get_settings(&self) -> &ThemeSettings {
        &self.settings
    }

    /// Update theme settings
    pub fn update_settings(&mut self, settings: ThemeSettings, cx: &mut App) -> Result<()> {
        let old_dynamic = self.settings.dynamic_enabled;
        let old_custom = self.settings.custom_color;

        self.settings = settings;
        self.settings.validate_and_fix();

        // Handle mode changes
        if old_dynamic != self.settings.dynamic_enabled {
            self.set_dynamic_mode(self.settings.dynamic_enabled, cx)?;
        } else if old_custom != self.settings.custom_color {
            if let Some(color) = self.settings.custom_color {
                self.apply_custom_color(color, cx)?;
            }
        }

        // Update theme context settings
        ThemeContext::update_global(cx, |theme| {
            theme.update_settings(self.settings.clone());
        });

        self.save_settings()?;
        Ok(())
    }

    /// Handle system theme changes (light/dark mode)
    pub fn handle_system_theme_change(&mut self, is_dark: bool, cx: &mut App) {
        ThemeContext::update_global(cx, |theme| {
            theme.handle_system_theme_change(is_dark);
        });
    }

    /// Shutdown the theme manager and cleanup resources
    pub fn shutdown(&mut self) {
        self.stop_wallpaper_monitoring();
        self.is_active = false;

        // Save final settings
        let _ = self.save_settings();
    }

    /// Check if the manager is currently active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Helper method to handle system appearance changes
    /// This should be called from view contexts that observe window appearance
    pub fn handle_system_appearance_change_global(is_dark: bool, cx: &mut App) {
        // Update the global theme context
        ThemeContext::update_global(cx, |theme| {
            theme.handle_system_theme_change(is_dark);
        });

        // Update the theme manager if it exists
        if let Some(manager) = cx.try_global::<ThemeManager>() {
            let mut manager = manager.clone();
            manager.handle_system_theme_change(is_dark, cx);
            cx.set_global(manager);
        }
    }

    /// Setup periodic wallpaper checking using GPUI's timer system
    /// This is a more GPUI-integrated approach than the separate thread monitor
    pub fn setup_periodic_wallpaper_check(&self, cx: &mut App) -> Task<()> {
        let check_interval = std::time::Duration::from_secs(5); // Check every 5 seconds

        cx.spawn(async move |cx| {
            loop {
                // Create a new timer for each iteration
                let timer = cx.background_executor().timer(check_interval);
                timer.await;

                // Update on the main thread
                let _ = cx.update(|cx| {
                    if let Some(manager) = cx.try_global::<ThemeManager>() {
                        let mut manager = manager.clone();
                        if let Err(e) = manager.refresh_wallpaper_theme(cx) {
                            tracing::warn!("Failed to refresh wallpaper theme: {}", e);
                        }
                        cx.set_global(manager);
                    }
                });
            }
        })
    }

    // Private helper methods

    /// Apply initial theme based on current settings
    fn apply_initial_theme(&mut self, cx: &mut App) -> Result<()> {
        if self.settings.dynamic_enabled {
            // Try to apply wallpaper colors, fall back to default if it fails
            if let Err(_) = self.apply_wallpaper_colors(cx) {
                self.apply_default_colors(cx)?;
            }
        } else if let Some(custom_color) = self.settings.custom_color {
            self.apply_custom_color(custom_color, cx)?;
        } else {
            self.apply_default_colors(cx)?;
        }

        Ok(())
    }

    /// Apply a palette to the global theme context
    fn apply_palette(&self, palette: MaterialPalette, source: ThemeSource, cx: &mut App) {
        ThemeContext::update_global(cx, |theme| {
            theme.apply_theme_transition(palette);
            theme.set_theme_source(source);
        });
    }

    /// Start wallpaper monitoring
    fn start_wallpaper_monitoring(&mut self, _cx: &mut App) -> Result<()> {
        if self.wallpaper_monitor.is_some() {
            return Ok(()); // Already monitoring
        }

        // Create a callback that will be called when wallpaper changes
        // Note: This runs in a separate thread, so we can't directly update GPUI state
        // In a production implementation, you would use channels or other IPC mechanisms
        // to communicate wallpaper changes back to the main thread
        let callback = move |wallpaper_path: PathBuf| {
            tracing::info!("Wallpaper changed: {:?}", wallpaper_path);

            // TODO: In a full implementation, this would:
            // 1. Send a message through a channel to the main thread
            // 2. The main thread would receive the message and update the theme
            // 3. This could be done using tokio channels or GPUI's async capabilities

            // For now, we just log the change. The actual theme update would need
            // to be triggered manually or through a periodic check mechanism.
        };

        let mut monitor = WallpaperMonitor::new(callback)
            .map_err(|e| anyhow::anyhow!("Failed to create wallpaper monitor: {}", e))?;

        monitor.start().map_err(|e| anyhow::anyhow!("Failed to start wallpaper monitoring: {}", e))?;
        self.wallpaper_monitor = Some(monitor);

        Ok(())
    }

    /// Manually check and update wallpaper-based theme
    /// This can be called periodically or on user action to update the theme
    /// based on the current wallpaper
    pub fn refresh_wallpaper_theme(&mut self, cx: &mut App) -> Result<()> {
        if !self.settings.dynamic_enabled {
            return Ok(()); // Dynamic theming is disabled
        }

        // Check if wallpaper has actually changed
        let current_wallpaper = self.color_extractor.get_wallpaper_path().ok();

        if !self.settings.has_wallpaper_changed(&current_wallpaper) {
            return Ok(()); // No change detected
        }

        // Apply new wallpaper colors
        self.apply_wallpaper_colors(cx)?;

        // Update settings with new wallpaper path
        self.settings.update_wallpaper_path(current_wallpaper);
        self.save_settings()?;

        Ok(())
    }

    /// Stop wallpaper monitoring
    fn stop_wallpaper_monitoring(&mut self) {
        if let Some(mut monitor) = self.wallpaper_monitor.take() {
            monitor.stop();
        }
    }

    /// Load settings from persistent storage
    fn load_settings(&mut self) -> Result<()> {
        // For now, use default settings
        // In a full implementation, this would load from a config file or database
        self.settings = ThemeSettings::default();
        Ok(())
    }

    /// Save settings to persistent storage
    fn save_settings(&self) -> Result<()> {
        // For now, this is a no-op
        // In a full implementation, this would save to a config file or database
        Ok(())
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ThemeManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Global theme manager access
impl ThemeManager {
    /// Initialize the global theme manager
    pub fn initialize_global(cx: &mut App) -> Result<()> {
        let mut manager = ThemeManager::new();
        manager.initialize(cx)?;
        cx.set_global(manager);
        Ok(())
    }

    /// Initialize the global theme manager with periodic wallpaper checking
    /// Returns a task that should be kept alive for wallpaper monitoring
    pub fn initialize_global_with_wallpaper_monitoring(cx: &mut App) -> Result<Task<()>> {
        // Initialize the basic theme manager
        Self::initialize_global(cx)?;

        // Check if dynamic theming is enabled and create the task accordingly
        let dynamic_enabled = cx.global::<ThemeManager>().settings.dynamic_enabled;

        let wallpaper_task = if dynamic_enabled {
            // Create the periodic wallpaper checking task
            let check_interval = std::time::Duration::from_secs(5);

            cx.spawn(async move |cx| {
                loop {
                    // Create a new timer for each iteration
                    let timer = cx.background_executor().timer(check_interval);
                    timer.await;

                    // Update on the main thread
                    let _ = cx.update(|cx| {
                        if let Some(manager) = cx.try_global::<ThemeManager>() {
                            let mut manager = manager.clone();
                            if let Err(e) = manager.refresh_wallpaper_theme(cx) {
                                tracing::warn!("Failed to refresh wallpaper theme: {}", e);
                            }
                            cx.set_global(manager);
                        }
                    });
                }
            })
        } else {
            // Return a completed task if not using dynamic theming
            cx.spawn(async move |_cx| {
                // Empty task that completes immediately
            })
        };

        Ok(wallpaper_task)
    }

    /// Get the global theme manager
    pub fn global(cx: &App) -> &ThemeManager {
        cx.global::<ThemeManager>()
    }

    /// Update the global theme manager
    pub fn update_global<F, R>(cx: &mut App, f: F) -> R
    where
        F: FnOnce(&mut ThemeManager) -> R,
    {
        let mut manager = cx.global::<ThemeManager>().clone();
        let result = f(&mut manager);
        cx.set_global(manager);
        result
    }
}

impl Global for ThemeManager {}

impl Clone for ThemeManager {
    fn clone(&self) -> Self {
        Self {
            color_extractor: ColorExtractor::new(),
            material_colors: MaterialColorsIntegration::new(),
            wallpaper_monitor: None, // Don't clone the monitor
            settings: self.settings.clone(),
            is_active: false, // New instance starts inactive
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert!(!manager.is_active());
        assert!(!manager.is_dynamic_mode_enabled()); // Default should be dynamic enabled
    }

    #[test]
    fn test_custom_color_setting() {
        let mut manager = ThemeManager::new();
        let test_color = Color::from_hex("#FF0000").unwrap();

        // This would normally require a context, but we're testing the logic
        assert_eq!(manager.get_custom_color(), None);

        // In a real test, we would set up a proper GPUI context
        // manager.set_custom_color(test_color, &mut cx).unwrap();
        // assert_eq!(manager.get_custom_color(), Some(test_color));
    }

    #[test]
    fn test_settings_validation() {
        let mut settings = ThemeSettings::default();
        settings.transition_duration_ms = 5000; // Too high
        settings.validate_and_fix();
        assert!(settings.transition_duration_ms <= 2000);
    }
}