//! Main application structure and GPUI app lifecycle management
//!
//! This module contains the core JellyfinApp struct and handles the GPUI
//! application lifecycle, window management, and global state initialization.

use std::sync::Arc;

use anyhow::Result;
use gpui::*;
use tracing::{debug, info};

use crate::{
    models::{AppConfig, AppState},
    services::ServiceManager,
    ui::{
        components::MainWindow,
        theme::setup_theme,
    },
    utils::error::AppError,
};

/// Main application struct that manages the GPUI app lifecycle
pub struct JellyfinApp {
    /// Application configuration
    config: Arc<AppConfig>,
    /// Service manager for handling business logic
    service_manager: Arc<ServiceManager>,
}

impl JellyfinApp {
    /// Create a new JellyfinApp instance
    pub fn new() -> Self {
        let config = Arc::new(AppConfig::default());
        let service_manager = Arc::new(ServiceManager::new(config.clone()));

        Self {
            config,
            service_manager,
        }
    }

    /// Run the GPUI application
    pub async fn run() -> Result<()> {
        info!("Initializing GPUI application");

        Application::new().run(move |cx: &mut App| {
            // Initialize application
            let app = Self::new();
            if let Err(e) = app.initialize_app(cx) {
                tracing::error!("Failed to initialize app: {}", e);
                return;
            }

            // Create main window
            if let Err(e) = app.create_main_window(cx) {
                tracing::error!("Failed to create main window: {}", e);
                return;
            }
        });

        Ok(())
    }

    /// Initialize the GPUI application with global state and services
    fn initialize_app(&self, cx: &mut App) -> Result<()> {
        info!("Setting up application globals and services");

        // Setup theme system
        setup_theme(cx);

        // Initialize global app state
        let app_state = AppState::new(self.config.clone());
        cx.set_global(app_state);

        // Register global actions and keybindings
        self.register_global_actions(cx);

        // Initialize service manager
        cx.set_global((*self.service_manager).clone());

        debug!("Application initialization complete");
        Ok(())
    }

    /// Create and configure the main application window
    fn create_main_window(&self, cx: &mut App) -> Result<()> {
        info!("Creating main application window");

        let window_bounds = Bounds::centered(
            None,
            size(px(1200.0), px(800.0)),
            cx,
        );

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            window_background: WindowBackgroundAppearance::Opaque,
            window_decorations: Some(WindowDecorations::Client),
            window_min_size: Some(size(px(800.0), px(600.0))),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Crabfin - Jellyfin Client")),
                appears_transparent: true,
                traffic_light_position: Some(Point {
                    x: px(12.0),
                    y: px(11.0),
                }),
            }),
            app_id: Some("com.crabfin.app".to_string()),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_window_title("Crabfin - Jellyfin Client");

            // Create the main window component
            cx.new(|cx| {
                // Observe window appearance changes for theme updates
                cx.observe_window_appearance(window, |_, _, cx| {
                    cx.refresh_windows();
                }).detach();

                // Handle app quit events
                cx.on_app_quit(move |_, _cx| async move {
                    info!("Application shutting down");
                    // TODO: Save application state and cleanup resources
                }).detach();

                MainWindow::new(cx)
            })
        })?;

        debug!("Main window created successfully");
        Ok(())
    }

    /// Register global actions and keybindings
    fn register_global_actions(&self, _cx: &mut App) {
        debug!("Registering global actions");

        // TODO: Register application-wide actions and keybindings
        // This will be expanded as we add more functionality

        // Example action registration (to be implemented):
        // cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    }
}

impl Default for JellyfinApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Global trait implementation for accessing the app instance
impl Global for JellyfinApp {}

/// Initialize and run the Jellyfin application
pub async fn run_jellyfin_app() -> Result<()> {
    JellyfinApp::run().await.map_err(|e| {
        AppError::InitializationError(format!("Failed to run GPUI application: {}", e)).into()
    })
}