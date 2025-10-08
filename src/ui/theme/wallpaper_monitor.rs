use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Wallpaper monitor that detects wallpaper changes and provides debouncing
pub struct WallpaperMonitor {
    callback: Arc<dyn Fn(PathBuf) + Send + Sync>,
    debounce_duration: Duration,
    last_update: Arc<Mutex<Option<Instant>>>,
    is_monitoring: bool,
    stop_sender: Option<mpsc::Sender<()>>,
}

impl WallpaperMonitor {
    /// Create a new wallpaper monitor with the given callback
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn(PathBuf) + Send + Sync + 'static,
    {
        Ok(Self {
            callback: Arc::new(callback),
            debounce_duration: Duration::from_millis(500), // 500ms debounce
            last_update: Arc::new(Mutex::new(None)),
            is_monitoring: false,
            stop_sender: None,
        })
    }

    /// Set custom debounce duration
    pub fn with_debounce_duration(mut self, duration: Duration) -> Self {
        self.debounce_duration = duration;
        self
    }

    /// Start monitoring wallpaper changes
    pub fn start(&mut self) -> Result<()> {
        if self.is_monitoring {
            return Ok(());
        }

        let callback = self.callback.clone();
        let last_update = self.last_update.clone();
        let debounce_duration = self.debounce_duration;
        let (stop_tx, stop_rx) = mpsc::channel();

        // Get initial wallpaper to establish baseline
        let mut last_wallpaper = Self::get_current_wallpaper_internal()?;

        thread::spawn(move || {
            loop {
                // Check if we should stop
                if stop_rx.try_recv().is_ok() {
                    break;
                }

                // Poll for wallpaper changes
                if let Ok(current_wallpaper) = Self::get_current_wallpaper_internal() {
                    if current_wallpaper != last_wallpaper {
                        // Wallpaper changed, apply debouncing
                        let now = Instant::now();
                        let mut last_update_guard = last_update.lock().unwrap();

                        // Check if enough time has passed since last update
                        let should_update = if let Some(last) = *last_update_guard {
                            now.duration_since(last) >= debounce_duration
                        } else {
                            true // First update
                        };

                        if should_update {
                            *last_update_guard = Some(now);
                            drop(last_update_guard);

                            // Update our tracking and call the callback
                            last_wallpaper = current_wallpaper.clone();
                            callback(current_wallpaper);
                        }
                    }
                }

                // Poll every 2 seconds
                thread::sleep(Duration::from_secs(2));
            }
        });

        self.stop_sender = Some(stop_tx);
        self.is_monitoring = true;
        Ok(())
    }

    /// Stop monitoring wallpaper changes
    pub fn stop(&mut self) {
        if let Some(sender) = self.stop_sender.take() {
            let _ = sender.send(());
        }
        self.is_monitoring = false;
    }

    /// Get the current wallpaper path
    pub fn get_current_wallpaper(&self) -> Result<PathBuf> {
        Self::get_current_wallpaper_internal()
    }

    /// Check if currently monitoring
    pub fn is_monitoring(&self) -> bool {
        self.is_monitoring
    }

    /// Internal method to get current wallpaper using the wallpaper crate
    fn get_current_wallpaper_internal() -> Result<PathBuf> {
        match wallpaper::get() {
            Ok(path) => Ok(PathBuf::from(path)),
            Err(e) => Err(anyhow::anyhow!("Failed to get wallpaper: {}", e)),
        }
    }
}

impl Drop for WallpaperMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_wallpaper_monitor_creation() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let monitor = WallpaperMonitor::new(move |_path| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(monitor.is_ok());
        let monitor = monitor.unwrap();
        assert!(!monitor.is_monitoring());
    }

    #[test]
    fn test_get_current_wallpaper() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let monitor = WallpaperMonitor::new(move |_path| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }).unwrap();

        // This test might fail on systems without wallpaper set
        // but it should at least not panic
        let _result = monitor.get_current_wallpaper();
    }

    #[test]
    fn test_custom_debounce_duration() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let monitor = WallpaperMonitor::new(move |_path| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        })
            .unwrap()
            .with_debounce_duration(Duration::from_millis(100));

        assert!(!monitor.is_monitoring());
    }
}