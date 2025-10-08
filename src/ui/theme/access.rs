use crate::ui::theme::{AccentColors, Color, SurfaceColorSet, TextColors, ThemeContext};
use gpui::*;

/// Trait for components that need theme access
pub trait ThemedComponent {
    /// Get current surface colors
    fn surface_colors(&self, cx: &App) -> SurfaceColorSet {
        cx.global::<ThemeContext>().get_surface_colors()
    }

    /// Get current text colors
    fn text_colors(&self, cx: &App) -> TextColors {
        cx.global::<ThemeContext>().get_text_colors()
    }

    /// Get current accent colors
    fn accent_colors(&self, cx: &App) -> AccentColors {
        cx.global::<ThemeContext>().get_accent_colors()
    }

    /// Check if current theme is dark mode
    fn is_dark_mode(&self, cx: &App) -> bool {
        cx.global::<ThemeContext>().is_dark_mode()
    }
}

/// Global theme access functions for use in render methods
pub struct ThemeAccess;

impl ThemeAccess {
    /// Get surface colors for the current theme
    pub fn surface_colors(cx: &App) -> SurfaceColorSet {
        cx.global::<ThemeContext>().get_surface_colors()
    }

    /// Get text colors for the current theme
    pub fn text_colors(cx: &App) -> TextColors {
        cx.global::<ThemeContext>().get_text_colors()
    }

    /// Get accent colors for the current theme
    pub fn accent_colors(cx: &App) -> AccentColors {
        cx.global::<ThemeContext>().get_accent_colors()
    }

    /// Check if the current theme is in dark mode
    pub fn is_dark_mode(cx: &App) -> bool {
        cx.global::<ThemeContext>().is_dark_mode()
    }

    /// Get the primary surface color
    pub fn primary_surface(cx: &App) -> Color {
        Self::surface_colors(cx).surface
    }

    /// Get the primary text color
    pub fn primary_text(cx: &App) -> Color {
        Self::text_colors(cx).primary
    }

    /// Get the primary accent color
    pub fn primary_accent(cx: &App) -> Color {
        Self::accent_colors(cx).primary
    }

    /// Get the outline color for borders
    pub fn outline(cx: &App) -> Color {
        Self::surface_colors(cx).outline
    }

    /// Get the error color
    pub fn error(cx: &App) -> Color {
        Self::accent_colors(cx).error
    }

    /// Convert Color to GPUI Hsla for styling
    pub fn to_hsla(color: Color) -> Hsla {
        let (h, s, l) = color.to_hsl();
        hsla(h / 360.0, s, l, color.a)
    }

    /// Get surface color as GPUI Hsla
    pub fn surface_hsla(cx: &App) -> Hsla {
        Self::to_hsla(Self::primary_surface(cx))
    }

    /// Get text color as GPUI Hsla
    pub fn text_hsla(cx: &App) -> Hsla {
        Self::to_hsla(Self::primary_text(cx))
    }

    /// Get accent color as GPUI Hsla
    pub fn accent_hsla(cx: &App) -> Hsla {
        Self::to_hsla(Self::primary_accent(cx))
    }

    /// Get outline color as GPUI Hsla
    pub fn outline_hsla(cx: &App) -> Hsla {
        Self::to_hsla(Self::outline(cx))
    }
}

/// Convenience macros for theme access in render methods
#[macro_export]
macro_rules! theme_surface {
    ($cx:expr) => {
        $crate::ui::theme::access::ThemeAccess::surface_hsla($cx)
    };
}

#[macro_export]
macro_rules! theme_text {
    ($cx:expr) => {
        $crate::ui::theme::access::ThemeAccess::text_hsla($cx)
    };
}

#[macro_export]
macro_rules! theme_accent {
    ($cx:expr) => {
        $crate::ui::theme::access::ThemeAccess::accent_hsla($cx)
    };
}

#[macro_export]
macro_rules! theme_outline {
    ($cx:expr) => {
        $crate::ui::theme::access::ThemeAccess::outline_hsla($cx)
    };
}

/// Theme-aware styling utilities
pub struct ThemeStyles;

impl ThemeStyles {
    /// Create a themed surface container style
    pub fn surface_container<T>(cx: &Context<T>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();

        div()
            .bg(ThemeAccess::to_hsla(surface_colors.surface_container))
            .border_1()
            .border_color(ThemeAccess::to_hsla(surface_colors.outline_variant))
            .rounded_md()
    }

    /// Create a themed text element
    pub fn text<T>(content: impl Into<SharedString>, cx: &Context<T>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let text_colors = theme_context.get_text_colors();

        div()
            .text_color(ThemeAccess::to_hsla(text_colors.primary))
            .child(content.into())
    }

    /// Create a themed card element
    pub fn card<T>(cx: &Context<T>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();

        div()
            .bg(ThemeAccess::to_hsla(surface_colors.surface_container_high))
            .border_1()
            .border_color(ThemeAccess::to_hsla(surface_colors.outline_variant))
            .rounded_lg()
            .p_4()
    }
}

/// Extension trait for GPUI elements to add theme-aware styling
pub trait ThemedElement: IntoElement + Styled + Sized {
    /// Apply themed surface background
    fn themed_surface<T>(self, cx: &Context<T>) -> Self {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        self.bg(ThemeAccess::to_hsla(surface_colors.surface))
    }

    /// Apply themed text color
    fn themed_text<T>(self, cx: &Context<T>) -> Self {
        let theme_context = cx.global::<ThemeContext>();
        let text_colors = theme_context.get_text_colors();
        self.text_color(ThemeAccess::to_hsla(text_colors.primary))
    }

    /// Apply themed accent color
    fn themed_accent<T>(self, cx: &Context<T>) -> Self {
        let theme_context = cx.global::<ThemeContext>();
        let accent_colors = theme_context.get_accent_colors();
        self.bg(ThemeAccess::to_hsla(accent_colors.primary))
    }

    /// Apply themed border
    fn themed_border<T>(self, cx: &Context<T>) -> Self {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        self.border_1().border_color(ThemeAccess::to_hsla(surface_colors.outline))
    }
}

// Implement ThemedElement for common GPUI elements
impl<E: IntoElement + Styled> ThemedElement for E {}