use gpui::*;
use material_colors::{
    color::Argb,
    scheme::Scheme,
    theme::ThemeBuilder,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct Theme {
    pub scheme: Arc<Scheme>,
}

impl Global for Theme {}

impl Theme {
    pub fn new(source_color: Argb, is_dark: bool) -> Self {
        let builder = ThemeBuilder::with_source(source_color);
        let theme = builder.build();
        let scheme = if is_dark { theme.schemes.dark } else { theme.schemes.light };
        Self { scheme: Arc::new(scheme) }
    }

    // Helper to convert Argb to GPUI Hsla
    fn color(&self, argb: Argb) -> Hsla {
        let r = argb.red as f32 / 255.0;
        let g = argb.green as f32 / 255.0;
        let b = argb.blue as f32 / 255.0;
        let a = argb.alpha as f32 / 255.0;
        Rgba { r, g, b, a }.into()
    }

    // Accessors for common tokens
    pub fn primary(&self) -> Hsla { self.color(self.scheme.primary) }
    pub fn on_primary(&self) -> Hsla { self.color(self.scheme.on_primary) }
    pub fn primary_container(&self) -> Hsla { self.color(self.scheme.primary_container) }
    pub fn on_primary_container(&self) -> Hsla { self.color(self.scheme.on_primary_container) }

    pub fn secondary(&self) -> Hsla { self.color(self.scheme.secondary) }
    pub fn on_secondary(&self) -> Hsla { self.color(self.scheme.on_secondary) }
    pub fn secondary_container(&self) -> Hsla { self.color(self.scheme.secondary_container) }
    pub fn on_secondary_container(&self) -> Hsla { self.color(self.scheme.on_secondary_container) }

    pub fn tertiary(&self) -> Hsla { self.color(self.scheme.tertiary) }
    pub fn on_tertiary(&self) -> Hsla { self.color(self.scheme.on_tertiary) }
    pub fn tertiary_container(&self) -> Hsla { self.color(self.scheme.tertiary_container) }
    pub fn on_tertiary_container(&self) -> Hsla { self.color(self.scheme.on_tertiary_container) }

    pub fn error(&self) -> Hsla { self.color(self.scheme.error) }
    pub fn on_error(&self) -> Hsla { self.color(self.scheme.on_error) }
    pub fn error_container(&self) -> Hsla { self.color(self.scheme.error_container) }
    pub fn on_error_container(&self) -> Hsla { self.color(self.scheme.on_error_container) }

    pub fn background(&self) -> Hsla { self.color(self.scheme.background) }
    pub fn on_background(&self) -> Hsla { self.color(self.scheme.on_background) }

    pub fn surface(&self) -> Hsla { self.color(self.scheme.surface) }
    pub fn on_surface(&self) -> Hsla { self.color(self.scheme.on_surface) }

    pub fn surface_variant(&self) -> Hsla { self.color(self.scheme.surface_variant) }
    pub fn on_surface_variant(&self) -> Hsla { self.color(self.scheme.on_surface_variant) }

    pub fn outline(&self) -> Hsla { self.color(self.scheme.outline) }
    pub fn outline_variant(&self) -> Hsla { self.color(self.scheme.outline_variant) }

    pub fn shadow(&self) -> Hsla { self.color(self.scheme.shadow) }
    pub fn scrim(&self) -> Hsla { self.color(self.scheme.scrim) }
    pub fn inverse_surface(&self) -> Hsla { self.color(self.scheme.inverse_surface) }
    pub fn inverse_on_surface(&self) -> Hsla { self.color(self.scheme.inverse_on_surface) }
    pub fn inverse_primary(&self) -> Hsla { self.color(self.scheme.inverse_primary) }

    pub fn font_family(&self) -> SharedString {
        "Adwaita Sans".into()
    }
}

// Green 500: #4CAF50 -> 0xFF4CAF50
pub const SEED_COLOR: Argb = Argb::from_u32(0xFF4CAF50);

pub fn update_theme(cx: &mut App, is_dark: bool) {
    let theme = Theme::new(SEED_COLOR, is_dark);
    cx.set_global(theme);
}

pub fn setup_theme(cx: &mut App) {
    let window_appearance = cx.window_appearance();
    let is_dark = matches!(window_appearance, WindowAppearance::Dark | WindowAppearance::VibrantDark);
    update_theme(cx, is_dark);
}
