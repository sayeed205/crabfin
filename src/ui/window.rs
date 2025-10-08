//! Window management
//!
//! This module handles GPUI window creation and management.

use anyhow::Result;
use gpui::*;
use tracing::{debug, info};

use super::theme::setup_theme;
use super::views::MainView;

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub bounds: Bounds<Pixels>,
    pub min_size: Option<Size<Pixels>>,
    pub resizable: bool,
    pub maximizable: bool,
    pub minimizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Crabfin - Jellyfin Client".to_string(),
            bounds: Bounds {
                origin: Point { x: px(100.0), y: px(100.0) },
                size: Size { width: px(1200.0), height: px(800.0) },
            },
            min_size: Some(Size { width: px(800.0), height: px(600.0) }),
            resizable: true,
            maximizable: true,
            minimizable: true,
        }
    }
}

/// Window manager for handling application windows
pub struct WindowManager {
    main_window: Option<WindowHandle<MainView>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            main_window: None,
        }
    }

    /// Create and show the main application window
    pub fn create_main_window(&mut self, cx: &mut App) -> Result<WindowHandle<MainView>> {
        debug!("Creating main application window");

        // Setup theme system
        setup_theme(cx);

        let config = WindowConfig::default();

        let title = config.title.clone();
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(config.bounds)),
            window_min_size: config.min_size,
            titlebar: Some(TitlebarOptions {
                title: Some(title.clone().into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_decorations: Some(WindowDecorations::Server),
            window_background: WindowBackgroundAppearance::Opaque,
            app_id: Some("com.hitarashi.crabfin".to_string()),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        let window_handle = cx.open_window(window_options, move |window, cx| {
            info!("Initializing main window view");
            window.set_window_title(&title);

            cx.new(|cx| MainView::new(cx))
        })?;

        self.main_window = Some(window_handle.clone());

        info!("Main window created successfully");
        Ok(window_handle)
    }

    /// Get the main window handle
    pub fn main_window(&self) -> Option<&WindowHandle<MainView>> {
        self.main_window.as_ref()
    }

    /// Close the main window
    pub fn close_main_window(&mut self, _cx: &mut App) {
        if let Some(_window) = &self.main_window {
            debug!("Closing main window");
            // For now, just remove the reference - GPUI will handle cleanup
            self.main_window = None;
        }
    }

    /// Check if the main window is open
    pub fn is_main_window_open(&self) -> bool {
        self.main_window.is_some()
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a basic window with default settings
pub fn create_window<V>(
    cx: &mut App,
    title: String,
    bounds: Bounds<Pixels>,
    build_view: impl FnOnce(&mut Context<V>) -> V + 'static,
) -> Result<WindowHandle<V>>
where
    V: 'static + Render,
{
    let title_clone = title.clone();
    let window_options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions {
            title: Some(title.into()),
            appears_transparent: false,
            traffic_light_position: None,
        }),
        window_decorations: Some(WindowDecorations::Server),
        window_background: WindowBackgroundAppearance::Opaque,
        app_id: Some("org.crabfin.jellyfin-client".to_string()),
        kind: WindowKind::Normal,
        window_min_size: None,
        ..Default::default()
    };

    cx.open_window(window_options, move |window, cx| {
        window.set_window_title(&title_clone);
        cx.new(build_view)
    })
}

/// Helper function to get the primary display bounds
pub fn get_primary_display_bounds(cx: &App) -> Option<Bounds<Pixels>> {
    cx.displays()
        .into_iter()
        .next() // Just get the first display for now
        .map(|display| display.bounds())
}

/// Helper function to center a window on the primary display
pub fn center_window_on_display(window_size: Size<Pixels>, cx: &App) -> Bounds<Pixels> {
    if let Some(display_bounds) = get_primary_display_bounds(cx) {
        let x = display_bounds.origin.x + (display_bounds.size.width - window_size.width) / 2.0;
        let y = display_bounds.origin.y + (display_bounds.size.height - window_size.height) / 2.0;

        Bounds {
            origin: Point { x, y },
            size: window_size,
        }
    } else {
        // Fallback to default position
        Bounds {
            origin: Point { x: px(100.0), y: px(100.0) },
            size: window_size,
        }
    }
}