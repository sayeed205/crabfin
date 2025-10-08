use crate::ui::theme::Color;
use serde::{Deserialize, Serialize};

/// Material 3 color scheme for a single color role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub light: Color,
    pub dark: Color,
    pub container_light: Color,
    pub container_dark: Color,
    pub on_light: Color,
    pub on_dark: Color,
    pub on_container_light: Color,
    pub on_container_dark: Color,
}

impl ColorScheme {
    pub fn new(
        light: Color,
        dark: Color,
        container_light: Color,
        container_dark: Color,
        on_light: Color,
        on_dark: Color,
        on_container_light: Color,
        on_container_dark: Color,
    ) -> Self {
        Self {
            light,
            dark,
            container_light,
            container_dark,
            on_light,
            on_dark,
            on_container_light,
            on_container_dark,
        }
    }

    /// Get the appropriate color for the current theme mode
    pub fn get_color(&self, is_dark: bool) -> Color {
        if is_dark {
            self.dark
        } else {
            self.light
        }
    }

    /// Get the appropriate container color for the current theme mode
    pub fn get_container_color(&self, is_dark: bool) -> Color {
        if is_dark {
            self.container_dark
        } else {
            self.container_light
        }
    }

    /// Get the appropriate on-color for the current theme mode
    pub fn get_on_color(&self, is_dark: bool) -> Color {
        if is_dark {
            self.on_dark
        } else {
            self.on_light
        }
    }

    /// Get the appropriate on-container color for the current theme mode
    pub fn get_on_container_color(&self, is_dark: bool) -> Color {
        if is_dark {
            self.on_container_dark
        } else {
            self.on_container_light
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            light: Color::from_hex("#6750A4").unwrap_or_default(),
            dark: Color::from_hex("#D0BCFF").unwrap_or_default(),
            container_light: Color::from_hex("#EADDFF").unwrap_or_default(),
            container_dark: Color::from_hex("#4F378B").unwrap_or_default(),
            on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
            on_dark: Color::from_hex("#381E72").unwrap_or_default(),
            on_container_light: Color::from_hex("#21005D").unwrap_or_default(),
            on_container_dark: Color::from_hex("#EADDFF").unwrap_or_default(),
        }
    }
}

/// Complete Material 3 color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialPalette {
    pub primary: ColorScheme,
    pub secondary: ColorScheme,
    pub tertiary: ColorScheme,
    pub neutral: ColorScheme,
    pub neutral_variant: ColorScheme,
    pub error: ColorScheme,
    pub surface: SurfaceColors,
}

impl MaterialPalette {
    pub fn new(
        primary: ColorScheme,
        secondary: ColorScheme,
        tertiary: ColorScheme,
        neutral: ColorScheme,
        neutral_variant: ColorScheme,
        error: ColorScheme,
        surface: SurfaceColors,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
            surface,
        }
    }

    /// Get surface colors for the current theme mode
    pub fn get_surface_colors(&self, is_dark: bool) -> SurfaceColorSet {
        if is_dark {
            self.surface.dark.clone()
        } else {
            self.surface.light.clone()
        }
    }

    /// Get text colors for the current theme mode
    pub fn get_text_colors(&self, is_dark: bool) -> TextColors {
        TextColors {
            primary: self.neutral.get_on_color(is_dark),
            secondary: self.neutral_variant.get_on_color(is_dark),
            disabled: {
                let base = self.neutral.get_on_color(is_dark);
                Color::new(base.r, base.g, base.b, 0.38)
            },
            on_primary: self.primary.get_on_color(is_dark),
            on_secondary: self.secondary.get_on_color(is_dark),
            on_tertiary: self.tertiary.get_on_color(is_dark),
            on_error: self.error.get_on_color(is_dark),
        }
    }

    /// Get accent colors for the current theme mode
    pub fn get_accent_colors(&self, is_dark: bool) -> AccentColors {
        AccentColors {
            primary: self.primary.get_color(is_dark),
            secondary: self.secondary.get_color(is_dark),
            tertiary: self.tertiary.get_color(is_dark),
            error: self.error.get_color(is_dark),
        }
    }
}

impl Default for MaterialPalette {
    fn default() -> Self {
        Self {
            primary: ColorScheme::default(),
            secondary: ColorScheme {
                light: Color::from_hex("#625B71").unwrap_or_default(),
                dark: Color::from_hex("#CCC2DC").unwrap_or_default(),
                container_light: Color::from_hex("#E8DEF8").unwrap_or_default(),
                container_dark: Color::from_hex("#4A4458").unwrap_or_default(),
                on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_dark: Color::from_hex("#332D41").unwrap_or_default(),
                on_container_light: Color::from_hex("#1D192B").unwrap_or_default(),
                on_container_dark: Color::from_hex("#E8DEF8").unwrap_or_default(),
            },
            tertiary: ColorScheme {
                light: Color::from_hex("#7D5260").unwrap_or_default(),
                dark: Color::from_hex("#EFB8C8").unwrap_or_default(),
                container_light: Color::from_hex("#FFDADF").unwrap_or_default(),
                container_dark: Color::from_hex("#633B48").unwrap_or_default(),
                on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_dark: Color::from_hex("#492532").unwrap_or_default(),
                on_container_light: Color::from_hex("#31111D").unwrap_or_default(),
                on_container_dark: Color::from_hex("#FFDADF").unwrap_or_default(),
            },
            neutral: ColorScheme {
                light: Color::from_hex("#1C1B1F").unwrap_or_default(),
                dark: Color::from_hex("#E6E1E5").unwrap_or_default(),
                container_light: Color::from_hex("#F3EDF7").unwrap_or_default(),
                container_dark: Color::from_hex("#1C1B1F").unwrap_or_default(),
                on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_dark: Color::from_hex("#1C1B1F").unwrap_or_default(),
                on_container_light: Color::from_hex("#1C1B1F").unwrap_or_default(),
                on_container_dark: Color::from_hex("#E6E1E5").unwrap_or_default(),
            },
            neutral_variant: ColorScheme {
                light: Color::from_hex("#49454F").unwrap_or_default(),
                dark: Color::from_hex("#CAC4D0").unwrap_or_default(),
                container_light: Color::from_hex("#79747E").unwrap_or_default(),
                container_dark: Color::from_hex("#49454F").unwrap_or_default(),
                on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_dark: Color::from_hex("#1C1B1F").unwrap_or_default(),
                on_container_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_container_dark: Color::from_hex("#CAC4D0").unwrap_or_default(),
            },
            error: ColorScheme {
                light: Color::from_hex("#B3261E").unwrap_or_default(),
                dark: Color::from_hex("#F2B8B5").unwrap_or_default(),
                container_light: Color::from_hex("#F9DEDC").unwrap_or_default(),
                container_dark: Color::from_hex("#8C1D18").unwrap_or_default(),
                on_light: Color::from_hex("#FFFFFF").unwrap_or_default(),
                on_dark: Color::from_hex("#601410").unwrap_or_default(),
                on_container_light: Color::from_hex("#410E0B").unwrap_or_default(),
                on_container_dark: Color::from_hex("#F9DEDC").unwrap_or_default(),
            },
            surface: SurfaceColors::default(),
        }
    }
}

/// Surface colors for different elevations and contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceColors {
    pub light: SurfaceColorSet,
    pub dark: SurfaceColorSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceColorSet {
    pub surface: Color,
    pub surface_variant: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_container_low: Color,
    pub surface_container_lowest: Color,
    pub surface_bright: Color,
    pub surface_dim: Color,
    pub outline: Color,
    pub outline_variant: Color,
}

impl Default for SurfaceColors {
    fn default() -> Self {
        Self {
            light: SurfaceColorSet {
                surface: Color::from_hex("#FEF7FF").unwrap_or_default(),
                surface_variant: Color::from_hex("#E7E0EC").unwrap_or_default(),
                surface_container: Color::from_hex("#F3EDF7").unwrap_or_default(),
                surface_container_high: Color::from_hex("#ECE6F0").unwrap_or_default(),
                surface_container_highest: Color::from_hex("#E6E0E9").unwrap_or_default(),
                surface_container_low: Color::from_hex("#F7F2FA").unwrap_or_default(),
                surface_container_lowest: Color::from_hex("#FFFFFF").unwrap_or_default(),
                surface_bright: Color::from_hex("#FEF7FF").unwrap_or_default(),
                surface_dim: Color::from_hex("#DED8E1").unwrap_or_default(),
                outline: Color::from_hex("#79747E").unwrap_or_default(),
                outline_variant: Color::from_hex("#CAC4D0").unwrap_or_default(),
            },
            dark: SurfaceColorSet {
                surface: Color::from_hex("#141218").unwrap_or_default(),
                surface_variant: Color::from_hex("#49454F").unwrap_or_default(),
                surface_container: Color::from_hex("#211F26").unwrap_or_default(),
                surface_container_high: Color::from_hex("#2B2930").unwrap_or_default(),
                surface_container_highest: Color::from_hex("#36343B").unwrap_or_default(),
                surface_container_low: Color::from_hex("#1D1B20").unwrap_or_default(),
                surface_container_lowest: Color::from_hex("#0F0D13").unwrap_or_default(),
                surface_bright: Color::from_hex("#3B383E").unwrap_or_default(),
                surface_dim: Color::from_hex("#141218").unwrap_or_default(),
                outline: Color::from_hex("#938F99").unwrap_or_default(),
                outline_variant: Color::from_hex("#49454F").unwrap_or_default(),
            },
        }
    }
}

/// Text colors for different contexts
#[derive(Debug, Clone)]
pub struct TextColors {
    pub primary: Color,
    pub secondary: Color,
    pub disabled: Color,
    pub on_primary: Color,
    pub on_secondary: Color,
    pub on_tertiary: Color,
    pub on_error: Color,
}

/// Accent colors for interactive elements
#[derive(Debug, Clone)]
pub struct AccentColors {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub error: Color,
}