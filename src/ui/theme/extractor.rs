use crate::ui::theme::Color;
use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Extracts dominant colors from system wallpaper
pub struct ColorExtractor;

impl ColorExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract the best primary color from the current system wallpaper
    pub fn extract_from_wallpaper(&self) -> Result<Color> {
        let wallpaper_path = self.get_wallpaper_path()
            .context("Failed to get wallpaper path")?;

        let colors = self.analyze_image(&wallpaper_path)
            .context("Failed to analyze wallpaper image")?;

        Ok(self.select_primary_color(colors))
    }

    /// Get the current system wallpaper path
    pub fn get_wallpaper_path(&self) -> Result<PathBuf> {
        #[cfg(target_os = "linux")]
        {
            self.get_linux_wallpaper_path()
        }

        #[cfg(target_os = "macos")]
        {
            self.get_macos_wallpaper_path()
        }

        #[cfg(target_os = "windows")]
        {
            self.get_windows_wallpaper_path()
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            anyhow::bail!("Unsupported platform for wallpaper detection")
        }
    }

    /// Analyze an image and extract dominant colors
    pub fn analyze_image(&self, path: &Path) -> Result<Vec<Color>> {
        let img = ImageReader::open(path)
            .context("Failed to open image file")?
            .decode()
            .context("Failed to decode image")?;

        // Resize image for faster processing
        let img = img.resize(200, 200, image::imageops::FilterType::Lanczos3);

        self.extract_dominant_colors(&img)
    }

    /// Select the best primary color from a list of colors
    pub fn select_primary_color(&self, colors: Vec<Color>) -> Color {
        if colors.is_empty() {
            // Fallback to Material 3 default primary color
            return Color::from_hex("#6750A4").unwrap_or_default();
        }

        // Find the color with the best characteristics for a primary color:
        // - Good saturation (not too gray)
        // - Reasonable lightness (not too dark or too light)
        // - High frequency in the image
        colors
            .into_iter()
            .max_by(|a, b| {
                let score_a = self.calculate_primary_color_score(a);
                let score_b = self.calculate_primary_color_score(b);
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|| Color::from_hex("#6750A4").unwrap_or_default())
    }

    /// Extract dominant colors from an image using color quantization
    fn extract_dominant_colors(&self, img: &DynamicImage) -> Result<Vec<Color>> {
        let rgb_img = img.to_rgb8();
        let pixels = rgb_img.pixels();

        // Count color frequencies
        let mut color_counts: HashMap<(u8, u8, u8), u32> = HashMap::new();

        for pixel in pixels {
            let rgb = (pixel[0], pixel[1], pixel[2]);
            *color_counts.entry(rgb).or_insert(0) += 1;
        }

        // Group similar colors and find dominant ones
        let mut dominant_colors = Vec::new();
        let mut processed_colors = std::collections::HashSet::new();

        for ((r, g, b), count) in color_counts.iter() {
            if processed_colors.contains(&(*r, *g, *b)) {
                continue;
            }

            // Skip very dark or very light colors
            let luminance = (0.299 * *r as f32 + 0.587 * *g as f32 + 0.114 * *b as f32) / 255.0;
            if luminance < 0.1 || luminance > 0.9 {
                continue;
            }

            let color = Color::from_rgb_u8(*r, *g, *b);
            let (_, saturation, _) = color.to_hsl();

            // Skip colors with very low saturation (too gray)
            if saturation < 0.2 {
                continue;
            }

            // Group similar colors
            let mut group_count = *count;
            for ((or, og, ob), other_count) in color_counts.iter() {
                if processed_colors.contains(&(*or, *og, *ob)) {
                    continue;
                }

                let color_distance = self.calculate_color_distance((*r, *g, *b), (*or, *og, *ob));
                if color_distance < 30.0 {
                    group_count += other_count;
                    processed_colors.insert((*or, *og, *ob));
                }
            }

            // Only include colors that appear frequently enough
            if group_count > (rgb_img.pixels().len() as u32) / 100 {
                dominant_colors.push((color, group_count));
            }

            processed_colors.insert((*r, *g, *b));
        }

        // Sort by frequency and return top colors
        dominant_colors.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(dominant_colors.into_iter().take(10).map(|(color, _)| color).collect())
    }

    /// Calculate color distance in RGB space
    fn calculate_color_distance(&self, color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f32 {
        let dr = color1.0 as f32 - color2.0 as f32;
        let dg = color1.1 as f32 - color2.1 as f32;
        let db = color1.2 as f32 - color2.2 as f32;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// Calculate a score for how suitable a color is as a primary color
    fn calculate_primary_color_score(&self, color: &Color) -> f32 {
        let (_, saturation, lightness) = color.to_hsl();

        // Prefer colors with good saturation
        let saturation_score = if saturation > 0.3 && saturation < 0.9 {
            saturation
        } else {
            saturation * 0.5
        };

        // Prefer colors with moderate lightness
        let lightness_score = if lightness > 0.2 && lightness < 0.8 {
            1.0 - (lightness - 0.5).abs() * 2.0
        } else {
            0.3
        };

        saturation_score * lightness_score
    }

    #[cfg(target_os = "linux")]
    fn get_linux_wallpaper_path(&self) -> Result<PathBuf> {
        use std::process::Command;

        // Try GNOME first
        if let Ok(output) = Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.background", "picture-uri"])
            .output()
        {
            if output.status.success() {
                let uri = String::from_utf8_lossy(&output.stdout);
                let uri = uri.trim().trim_matches('\'').trim_matches('"');

                if uri.starts_with("file://") {
                    let path = uri.strip_prefix("file://").unwrap();
                    return Ok(PathBuf::from(path));
                }
            }
        }

        // Try KDE
        if let Some(home) = dirs::home_dir() {
            let kde_config = home.join(".config/plasma-org.kde.plasma.desktop-appletsrc");
            if kde_config.exists() {
                if let Ok(content) = std::fs::read_to_string(&kde_config) {
                    for line in content.lines() {
                        if line.contains("Image=") {
                            if let Some(path) = line.split("Image=").nth(1) {
                                let path = path.trim();
                                if !path.is_empty() && Path::new(path).exists() {
                                    return Ok(PathBuf::from(path));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Try common wallpaper locations
        if let Some(home) = dirs::home_dir() {
            let common_paths = [
                home.join("Pictures/wallpaper.jpg"),
                home.join("Pictures/wallpaper.png"),
                home.join(".config/nitrogen/bg-saved.cfg"),
            ];

            for path in &common_paths {
                if path.exists() {
                    if path.file_name().unwrap() == "bg-saved.cfg" {
                        // Parse nitrogen config
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for line in content.lines() {
                                if line.starts_with("file=") {
                                    let wallpaper_path = line.strip_prefix("file=").unwrap();
                                    return Ok(PathBuf::from(wallpaper_path));
                                }
                            }
                        }
                    } else {
                        return Ok(path.clone());
                    }
                }
            }
        }

        anyhow::bail!("Could not find wallpaper path on Linux")
    }

    #[cfg(target_os = "macos")]
    fn get_macos_wallpaper_path(&self) -> Result<PathBuf> {
        use std::process::Command;

        let output = Command::new("osascript")
            .args(&[
                "-e",
                "tell application \"Finder\" to get POSIX path of (get desktop picture as alias)"
            ])
            .output()
            .context("Failed to execute osascript")?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout);
            let path = path.trim();
            Ok(PathBuf::from(path))
        } else {
            anyhow::bail!("Failed to get macOS wallpaper path")
        }
    }

    #[cfg(target_os = "windows")]
    fn get_windows_wallpaper_path(&self) -> Result<PathBuf> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let desktop_key = hkcu
            .open_subkey("Control Panel\\Desktop")
            .context("Failed to open desktop registry key")?;

        let wallpaper_path: String = desktop_key
            .get_value("Wallpaper")
            .context("Failed to get wallpaper path from registry")?;

        if wallpaper_path.is_empty() {
            anyhow::bail!("Wallpaper path is empty");
        }

        Ok(PathBuf::from(wallpaper_path))
    }
}

impl Default for ColorExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_distance_calculation() {
        let extractor = ColorExtractor::new();

        // Same colors should have distance 0
        let distance = extractor.calculate_color_distance((255, 0, 0), (255, 0, 0));
        assert_eq!(distance, 0.0);

        // Different colors should have positive distance
        let distance = extractor.calculate_color_distance((255, 0, 0), (0, 255, 0));
        assert!(distance > 0.0);
    }

    #[test]
    fn test_primary_color_score() {
        let extractor = ColorExtractor::new();

        // Highly saturated, moderate lightness color should score well
        let good_color = Color::from_hsl(240.0, 0.7, 0.5);
        let good_score = extractor.calculate_primary_color_score(&good_color);

        // Gray color should score poorly
        let gray_color = Color::from_hsl(0.0, 0.0, 0.5);
        let gray_score = extractor.calculate_primary_color_score(&gray_color);

        assert!(good_score > gray_score);
    }

    #[test]
    fn test_select_primary_color_empty() {
        let extractor = ColorExtractor::new();
        let result = extractor.select_primary_color(vec![]);

        // Should return default Material 3 primary color
        assert_eq!(result, Color::from_hex("#6750A4").unwrap());
    }
}