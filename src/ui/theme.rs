//! Theme system for the application
//!
//! This module handles theme management, color schemes, and styling
//! for the GPUI interface.

use gpui::*;
use serde::{Deserialize, Serialize};

/// Application theme configuration
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
}

impl Global for Theme {}

/// Setup the theme system in the GPUI app
pub fn setup_theme(cx: &mut App) {
    // For now, use the default dark theme
    // TODO: Load theme from configuration or system preferences
    let theme = Theme::default();
    cx.set_global(theme);

    tracing::debug!("Theme system initialized");
}