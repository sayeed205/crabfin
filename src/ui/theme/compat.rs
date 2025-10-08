use crate::ui::theme::{Color, MaterialPalette};
use gpui::*;
use serde::{Deserialize, Serialize};

/// Legacy theme structure for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Primary background color
    pub background_primary: Hsla,
    /// Secondary background color
    pub background_secondary: Hsla,
    /// Primary text color
    pub text_primary: Hsla,
    /// Secondary text color
    pub text_secondary: Hsla,
    /// Accent color for highlights and selections
    pub accent: Hsla,
    /// Border color
    pub border: Hsla,
    /// Success color
    pub success: Hsla,
    /// Warning color
    pub warning: Hsla,
    /// Error color
    pub error: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl Theme {
    /// Create a dark theme
    pub fn dark_theme() -> Self {
        Self {
            background_primary: hsla(0.0, 0.0, 0.08, 1.0),    // Very dark gray
            background_secondary: hsla(0.0, 0.0, 0.12, 1.0),  // Slightly lighter dark gray
            text_primary: hsla(0.0, 0.0, 0.95, 1.0),          // Almost white
            text_secondary: hsla(0.0, 0.0, 0.7, 1.0),         // Light gray
            accent: hsla(210.0, 1.0, 0.6, 1.0),               // Blue accent
            border: hsla(0.0, 0.0, 0.2, 1.0),                 // Dark border
            success: hsla(120.0, 0.6, 0.5, 1.0),              // Green
            warning: hsla(45.0, 1.0, 0.6, 1.0),               // Orange
            error: hsla(0.0, 0.8, 0.6, 1.0),                  // Red
        }
    }

    /// Create a light theme
    pub fn light_theme() -> Self {
        Self {
            background_primary: hsla(0.0, 0.0, 0.98, 1.0),    // Almost white
            background_secondary: hsla(0.0, 0.0, 0.94, 1.0),  // Light gray
            text_primary: hsla(0.0, 0.0, 0.1, 1.0),           // Very dark gray
            text_secondary: hsla(0.0, 0.0, 0.4, 1.0),         // Medium gray
            accent: hsla(210.0, 1.0, 0.5, 1.0),               // Blue accent
            border: hsla(0.0, 0.0, 0.8, 1.0),                 // Light border
            success: hsla(120.0, 0.6, 0.4, 1.0),              // Green
            warning: hsla(45.0, 1.0, 0.5, 1.0),               // Orange
            error: hsla(0.0, 0.8, 0.5, 1.0),                  // Red
        }
    }

    /// Create theme from Material palette
    pub fn from_material_palette(palette: &MaterialPalette, is_dark: bool) -> Self {
        let surface_colors = palette.get_surface_colors(is_dark);
        let text_colors = palette.get_text_colors(is_dark);
        let accent_colors = palette.get_accent_colors(is_dark);

        // Convert Color to Hsla
        let to_hsla = |color: Color| -> Hsla {
            let (h, s, l) = color.to_hsl();
            hsla(h / 360.0, s, l, color.a)
        };

        Self {
            background_primary: to_hsla(surface_colors.surface),
            background_secondary: to_hsla(surface_colors.surface_variant),
            text_primary: to_hsla(text_colors.primary),
            text_secondary: to_hsla(text_colors.secondary),
            accent: to_hsla(accent_colors.primary),
            border: to_hsla(surface_colors.outline),
            success: hsla(120.0, 0.6, if is_dark { 0.5 } else { 0.4 }, 1.0),
            warning: hsla(45.0, 1.0, if is_dark { 0.6 } else { 0.5 }, 1.0),
            error: to_hsla(accent_colors.error),
        }
    }
}

impl Global for Theme {}

/// Setup the theme system in the GPUI app
pub fn setup_theme(cx: &mut App) {
    use crate::ui::theme::context::{SystemThemeDetector, ThemeContext};

    // Initialize the reactive theme context
    let mut theme_context = ThemeContext::new();

    // Detect initial system theme
    let is_dark = SystemThemeDetector::is_system_dark_mode();
    theme_context.is_dark_mode = is_dark;

    // Set as global
    cx.set_global(theme_context);

    // Start system theme monitoring
    if let Err(e) = SystemThemeDetector::start_monitoring(move |is_dark| {
        // Note: In a real implementation, we'd need to properly handle the context update
        // This is a simplified version for demonstration
        tracing::debug!("System theme changed to: {}", if is_dark { "dark" } else { "light" });
    }) {
        tracing::warn!("Failed to start system theme monitoring: {}", e);
    }

    // Setup legacy theme for backward compatibility
    let legacy_theme = Theme::default();
    cx.set_global(legacy_theme);

    tracing::debug!("Theme system initialized with reactive context");
}