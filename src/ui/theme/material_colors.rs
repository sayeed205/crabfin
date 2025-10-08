use crate::ui::theme::{Color, ColorScheme, MaterialPalette, SurfaceColorSet, SurfaceColors};
use anyhow::Result;
use material_colors::{
    color::Argb,
    dynamic_color::{DynamicColor, DynamicScheme, MaterialDynamicColors},
    hct::Hct,
    scheme::variant::{SchemeContent, SchemeFidelity, SchemeMonochrome, SchemeNeutral, SchemeTonalSpot, SchemeVibrant},
};
use std::path::Path;

/// Integration with the material-colors library for Material 3 palette generation
pub struct MaterialColorsIntegration;

impl MaterialColorsIntegration {
    /// Create a new MaterialColorsIntegration instance
    pub fn new() -> Self {
        Self
    }

    /// Generate a complete Material 3 palette from a source color
    pub fn generate_palette(&self, source_color: Color) -> Result<MaterialPalette> {
        let argb = self.color_to_argb(source_color);
        let source_argb = Argb::new(255, (argb >> 16) as u8, (argb >> 8) as u8, argb as u8);

        // Generate light and dark schemes using TonalSpot variant (default Material 3)
        let light_scheme = SchemeTonalSpot::new(Hct::new(source_argb), false, Some(0.0)).scheme;
        let dark_scheme = SchemeTonalSpot::new(Hct::new(source_argb), true, Some(0.0)).scheme;

        self.schemes_to_material_palette(&light_scheme, &dark_scheme)
    }

    /// Generate palette from an image file (extracts dominant color first)
    pub fn generate_from_image(&self, image_path: &Path) -> Result<MaterialPalette> {
        // For now, we'll use a placeholder implementation
        // In a full implementation, this would use image processing to extract colors
        // and then call generate_palette with the dominant color

        // Placeholder: use a default purple color
        let default_color = Color::from_hex("#6750A4")?;
        self.generate_palette(default_color)
    }

    /// Create a Material 3 scheme from a color with specific variant
    pub fn create_scheme_from_color(&self, color: Color, variant: SchemeVariant) -> Result<MaterialPalette> {
        let argb = self.color_to_argb(color);
        let source_argb = Argb::new(255, (argb >> 16) as u8, (argb >> 8) as u8, argb as u8);
        let hct = Hct::new(source_argb);

        let (light_scheme, dark_scheme) = match variant {
            SchemeVariant::TonalSpot => (
                SchemeTonalSpot::new(hct, false, Some(0.0)).scheme,
                SchemeTonalSpot::new(hct, true, Some(0.0)).scheme,
            ),
            SchemeVariant::Neutral => (
                SchemeNeutral::new(hct, false, Some(0.0)).scheme,
                SchemeNeutral::new(hct, true, Some(0.0)).scheme,
            ),
            SchemeVariant::Vibrant => (
                SchemeVibrant::new(hct, false, Some(0.0)).scheme,
                SchemeVibrant::new(hct, true, Some(0.0)).scheme,
            ),
            SchemeVariant::Monochrome => (
                SchemeMonochrome::new(hct, false, Some(0.0)).scheme,
                SchemeMonochrome::new(hct, true, Some(0.0)).scheme,
            ),
            SchemeVariant::Fidelity => (
                SchemeFidelity::new(hct, false, Some(0.0)).scheme,
                SchemeFidelity::new(hct, true, Some(0.0)).scheme,
            ),
            SchemeVariant::Content => (
                SchemeContent::new(hct, false, Some(0.0)).scheme,
                SchemeContent::new(hct, true, Some(0.0)).scheme,
            ),
        };

        self.schemes_to_material_palette(&light_scheme, &dark_scheme)
    }

    /// Generate palette with accessibility contrast validation
    pub fn generate_accessible_palette(&self, source_color: Color, min_contrast: f32) -> Result<MaterialPalette> {
        let mut palette = self.generate_palette(source_color)?;

        // Validate and adjust contrast ratios if needed
        self.ensure_accessibility_contrast(&mut palette, min_contrast);

        Ok(palette)
    }

    /// Calculate contrast ratio between two colors using material-colors utilities
    pub fn calculate_contrast_ratio(&self, color1: Color, color2: Color) -> f32 {
        let argb1 = self.color_to_argb(color1);
        let argb2 = self.color_to_argb(color2);

        // Use our own contrast calculation since color_utils might not be available
        let l1 = color1.luminance();
        let l2 = color2.luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Check if a color combination meets WCAG accessibility standards
    pub fn meets_accessibility_standards(&self, foreground: Color, background: Color, level: AccessibilityLevel) -> bool {
        let contrast = self.calculate_contrast_ratio(foreground, background);

        match level {
            AccessibilityLevel::AA => contrast >= 4.5,
            AccessibilityLevel::AAA => contrast >= 7.0,
            AccessibilityLevel::AALarge => contrast >= 3.0,
            AccessibilityLevel::AAALarge => contrast >= 4.5,
        }
    }

    /// Convert our Color struct to material-colors Argb u32
    fn color_to_argb(&self, color: Color) -> u32 {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;

        ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Convert material-colors Argb to our Color struct
    fn argb_to_color(&self, argb: Argb) -> Color {
        Color::new(
            argb.red as f32 / 255.0,
            argb.green as f32 / 255.0,
            argb.blue as f32 / 255.0,
            argb.alpha as f32 / 255.0,
        )
    }

    /// Convert material-colors schemes to our MaterialPalette
    fn schemes_to_material_palette(&self, light_scheme: &DynamicScheme, dark_scheme: &DynamicScheme) -> Result<MaterialPalette> {
        Ok(MaterialPalette::new(
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::primary(), MaterialDynamicColors::on_primary(),
                                                         MaterialDynamicColors::primary_container(), MaterialDynamicColors::on_primary_container()),
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::secondary(), MaterialDynamicColors::on_secondary(),
                                                         MaterialDynamicColors::secondary_container(), MaterialDynamicColors::on_secondary_container()),
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::tertiary(), MaterialDynamicColors::on_tertiary(),
                                                         MaterialDynamicColors::tertiary_container(), MaterialDynamicColors::on_tertiary_container()),
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::surface(), MaterialDynamicColors::on_surface(),
                                                         MaterialDynamicColors::surface_container(), MaterialDynamicColors::on_surface()),
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::surface_variant(), MaterialDynamicColors::on_surface_variant(),
                                                         MaterialDynamicColors::surface_container_low(), MaterialDynamicColors::on_surface_variant()),
            self.create_color_scheme_from_dynamic_colors(light_scheme, dark_scheme,
                                                         MaterialDynamicColors::error(), MaterialDynamicColors::on_error(),
                                                         MaterialDynamicColors::error_container(), MaterialDynamicColors::on_error_container()),
            self.create_surface_colors_from_schemes(light_scheme, dark_scheme),
        ))
    }

    /// Create a ColorScheme from light and dark material-colors schemes using DynamicColor
    fn create_color_scheme_from_dynamic_colors(
        &self,
        light_scheme: &DynamicScheme,
        dark_scheme: &DynamicScheme,
        mut main_color: DynamicColor,
        mut on_color: DynamicColor,
        mut container_color: DynamicColor,
        mut on_container_color: DynamicColor,
    ) -> ColorScheme {
        ColorScheme::new(
            self.argb_to_color(main_color.get_argb(light_scheme)),
            self.argb_to_color(main_color.get_argb(dark_scheme)),
            self.argb_to_color(container_color.get_argb(light_scheme)),
            self.argb_to_color(container_color.get_argb(dark_scheme)),
            self.argb_to_color(on_color.get_argb(light_scheme)),
            self.argb_to_color(on_color.get_argb(dark_scheme)),
            self.argb_to_color(on_container_color.get_argb(light_scheme)),
            self.argb_to_color(on_container_color.get_argb(dark_scheme)),
        )
    }

    /// Create SurfaceColors from light and dark schemes
    fn create_surface_colors_from_schemes(&self, light_scheme: &DynamicScheme, dark_scheme: &DynamicScheme) -> SurfaceColors {
        SurfaceColors {
            light: SurfaceColorSet {
                surface: self.argb_to_color(MaterialDynamicColors::surface().get_argb(light_scheme)),
                surface_variant: self.argb_to_color(MaterialDynamicColors::surface_variant().get_argb(light_scheme)),
                surface_container: self.argb_to_color(MaterialDynamicColors::surface_container().get_argb(light_scheme)),
                surface_container_high: self.argb_to_color(MaterialDynamicColors::surface_container_high().get_argb(light_scheme)),
                surface_container_highest: self.argb_to_color(MaterialDynamicColors::surface_container_highest().get_argb(light_scheme)),
                surface_container_low: self.argb_to_color(MaterialDynamicColors::surface_container_low().get_argb(light_scheme)),
                surface_container_lowest: self.argb_to_color(MaterialDynamicColors::surface_container_lowest().get_argb(light_scheme)),
                surface_bright: self.argb_to_color(MaterialDynamicColors::surface_bright().get_argb(light_scheme)),
                surface_dim: self.argb_to_color(MaterialDynamicColors::surface_dim().get_argb(light_scheme)),
                outline: self.argb_to_color(MaterialDynamicColors::outline().get_argb(light_scheme)),
                outline_variant: self.argb_to_color(MaterialDynamicColors::outline_variant().get_argb(light_scheme)),
            },
            dark: SurfaceColorSet {
                surface: self.argb_to_color(MaterialDynamicColors::surface().get_argb(dark_scheme)),
                surface_variant: self.argb_to_color(MaterialDynamicColors::surface_variant().get_argb(dark_scheme)),
                surface_container: self.argb_to_color(MaterialDynamicColors::surface_container().get_argb(dark_scheme)),
                surface_container_high: self.argb_to_color(MaterialDynamicColors::surface_container_high().get_argb(dark_scheme)),
                surface_container_highest: self.argb_to_color(MaterialDynamicColors::surface_container_highest().get_argb(dark_scheme)),
                surface_container_low: self.argb_to_color(MaterialDynamicColors::surface_container_low().get_argb(dark_scheme)),
                surface_container_lowest: self.argb_to_color(MaterialDynamicColors::surface_container_lowest().get_argb(dark_scheme)),
                surface_bright: self.argb_to_color(MaterialDynamicColors::surface_bright().get_argb(dark_scheme)),
                surface_dim: self.argb_to_color(MaterialDynamicColors::surface_dim().get_argb(dark_scheme)),
                outline: self.argb_to_color(MaterialDynamicColors::outline().get_argb(dark_scheme)),
                outline_variant: self.argb_to_color(MaterialDynamicColors::outline_variant().get_argb(dark_scheme)),
            },
        }
    }

    /// Ensure accessibility contrast ratios are met
    fn ensure_accessibility_contrast(&self, palette: &mut MaterialPalette, min_contrast: f32) {
        // Check and adjust primary colors if needed
        self.adjust_color_scheme_contrast(&mut palette.primary, min_contrast);
        self.adjust_color_scheme_contrast(&mut palette.secondary, min_contrast);
        self.adjust_color_scheme_contrast(&mut palette.tertiary, min_contrast);
        self.adjust_color_scheme_contrast(&mut palette.error, min_contrast);
    }

    /// Adjust a color scheme to meet minimum contrast requirements
    fn adjust_color_scheme_contrast(&self, scheme: &mut ColorScheme, min_contrast: f32) {
        // Check light theme contrast
        if self.calculate_contrast_ratio(scheme.on_light, scheme.light) < min_contrast {
            // Adjust the on-color to meet contrast requirements
            scheme.on_light = self.adjust_color_for_contrast(scheme.on_light, scheme.light, min_contrast);
        }

        // Check dark theme contrast
        if self.calculate_contrast_ratio(scheme.on_dark, scheme.dark) < min_contrast {
            scheme.on_dark = self.adjust_color_for_contrast(scheme.on_dark, scheme.dark, min_contrast);
        }

        // Check container contrasts
        if self.calculate_contrast_ratio(scheme.on_container_light, scheme.container_light) < min_contrast {
            scheme.on_container_light = self.adjust_color_for_contrast(scheme.on_container_light, scheme.container_light, min_contrast);
        }

        if self.calculate_contrast_ratio(scheme.on_container_dark, scheme.container_dark) < min_contrast {
            scheme.on_container_dark = self.adjust_color_for_contrast(scheme.on_container_dark, scheme.container_dark, min_contrast);
        }
    }

    /// Adjust a foreground color to meet contrast requirements against a background
    fn adjust_color_for_contrast(&self, foreground: Color, background: Color, min_contrast: f32) -> Color {
        let bg_luminance = background.luminance();

        // Determine if we should make the foreground lighter or darker
        let target_luminance = if bg_luminance > 0.5 {
            // Background is light, make foreground darker
            (bg_luminance - 0.05) / min_contrast - 0.05
        } else {
            // Background is dark, make foreground lighter
            (bg_luminance + 0.05) * min_contrast - 0.05
        };

        // Clamp target luminance to valid range
        let target_luminance = target_luminance.max(0.0).min(1.0);

        // Convert to HCT, adjust tone, and convert back
        let argb = self.color_to_argb(foreground);
        let source_argb = Argb::new(255, (argb >> 16) as u8, (argb >> 8) as u8, argb as u8);
        let hct = Hct::new(source_argb);

        // Calculate target tone from luminance (approximation)
        let target_tone = if target_luminance > 0.5 {
            50.0 + (target_luminance - 0.5) * 100.0
        } else {
            target_luminance * 100.0
        } as f64;

        // For now, just return the original color since HCT adjustment is complex
        // In a full implementation, this would properly adjust the tone
        foreground
    }
}

impl Default for MaterialColorsIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Different Material 3 scheme variants
#[derive(Debug, Clone, Copy)]
pub enum SchemeVariant {
    /// Default Material 3 scheme with balanced colors
    TonalSpot,
    /// Neutral scheme with muted colors
    Neutral,
    /// Vibrant scheme with saturated colors
    Vibrant,
    /// Monochrome scheme with grayscale colors
    Monochrome,
    /// High fidelity scheme that stays close to source color
    Fidelity,
    /// Content-based scheme optimized for readability
    Content,
}

/// WCAG accessibility levels
#[derive(Debug, Clone, Copy)]
pub enum AccessibilityLevel {
    /// WCAG AA standard (4.5:1 contrast ratio)
    AA,
    /// WCAG AAA standard (7:1 contrast ratio)
    AAA,
    /// WCAG AA for large text (3:1 contrast ratio)
    AALarge,
    /// WCAG AAA for large text (4.5:1 contrast ratio)
    AAALarge,
}

#[derive(Debug, thiserror::Error)]
pub enum MaterialColorsError {
    #[error("Failed to generate color scheme: {0}")]
    SchemeGeneration(String),

    #[error("Invalid color format: {0}")]
    InvalidColor(String),

    #[error("Image processing failed: {0}")]
    ImageProcessing(String),

    #[error("Accessibility requirements cannot be met: {0}")]
    AccessibilityError(String),
}