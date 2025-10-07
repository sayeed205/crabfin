use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Format duration as human-readable string (e.g., "1h 23m 45s")
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Format file size as human-readable string (e.g., "1.2 GB")
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Get current timestamp as Unix epoch seconds
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Sanitize filename by removing invalid characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .collect()
}

/// Get application data directory
pub fn get_app_data_dir() -> Result<PathBuf> {
    let app_name = "crabfin";
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            Ok(Path::new(&appdata).join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name))
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            Ok(Path::new(&home).join("Library").join("Application Support").join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name))
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            Ok(Path::new(&xdg_data_home).join(app_name))
        } else if let Ok(home) = std::env::var("HOME") {
            Ok(Path::new(&home).join(".local").join("share").join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name))
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(PathBuf::from(".").join(app_name))
    }
}

/// Get application cache directory
pub fn get_app_cache_dir() -> Result<PathBuf> {
    let app_name = "crabfin";
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            Ok(Path::new(&localappdata).join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name).join("cache"))
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            Ok(Path::new(&home).join("Library").join("Caches").join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name).join("cache"))
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg_cache_home) = std::env::var("XDG_CACHE_HOME") {
            Ok(Path::new(&xdg_cache_home).join(app_name))
        } else if let Ok(home) = std::env::var("HOME") {
            Ok(Path::new(&home).join(".cache").join(app_name))
        } else {
            Ok(PathBuf::from(".").join(app_name).join("cache"))
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(PathBuf::from(".").join(app_name).join("cache"))
    }
}

/// Create directory if it doesn't exist
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// URL validation helper
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Normalize URL by removing trailing slash
pub fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

/// Generate a simple UUID-like string
pub fn generate_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    current_timestamp().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    
    format!("{:x}", hasher.finish())
}