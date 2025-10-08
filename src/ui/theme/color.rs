use serde::{Deserialize, Serialize};
use std::fmt;

/// Core color structure with RGBA components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Eq for Color {}

impl std::hash::Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert floats to bits for hashing
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.a.to_bits().hash(state);
    }
}

impl Color {
    /// Create a new color with RGBA values (0.0 to 1.0)
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new opaque color with RGB values (0.0 to 1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create a color from hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(ColorError::InvalidHexFormat);
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| ColorError::InvalidHexFormat)? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| ColorError::InvalidHexFormat)? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| ColorError::InvalidHexFormat)? as f32 / 255.0;

        Ok(Self::rgb(r, g, b))
    }

    /// Convert color to hex string
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    /// Convert color to HSL (Hue, Saturation, Lightness)
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return (0.0, 0.0, l); // Achromatic
        }

        // Saturation
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        // Hue
        let h = if max == self.r {
            ((self.g - self.b) / delta + if self.g < self.b { 6.0 } else { 0.0 }) / 6.0
        } else if max == self.g {
            ((self.b - self.r) / delta + 2.0) / 6.0
        } else {
            ((self.r - self.g) / delta + 4.0) / 6.0
        };

        (h * 360.0, s, l)
    }

    /// Create color from HSL values
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h / 360.0; // Normalize hue to 0-1

        if s == 0.0 {
            return Self::rgb(l, l, l); // Achromatic
        }

        let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
            let t = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };

            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        Self::rgb(r, g, b)
    }

    /// Convert to RGB values (0-255)
    pub fn to_rgb_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    /// Create color from RGB values (0-255)
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    /// Calculate luminance for contrast calculations
    pub fn luminance(&self) -> f32 {
        let to_linear = |c: f32| {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        0.2126 * to_linear(self.r) + 0.7152 * to_linear(self.g) + 0.0722 * to_linear(self.b)
    }

    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("Invalid hex color format")]
    InvalidHexFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.to_hex(), "#FF0000");
    }

    #[test]
    fn test_hsl_conversion() {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let (h, s, l) = red.to_hsl();
        assert_eq!(h, 0.0);
        assert_eq!(s, 1.0);
        assert_eq!(l, 0.5);

        let converted_back = Color::from_hsl(h, s, l);
        assert!((converted_back.r - red.r).abs() < 0.001);
        assert!((converted_back.g - red.g).abs() < 0.001);
        assert!((converted_back.b - red.b).abs() < 0.001);
    }
}