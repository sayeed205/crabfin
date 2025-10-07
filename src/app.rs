// Main application struct and GPUI app lifecycle

use anyhow::Result;
use gpui::*;
use tracing::{debug, info, error};

use crate::error::{AppError, UiError, ErrorHandler, UserFriendlyError};

/// Main application state structure
pub struct JellyfinApp {
    /// Application configuration and settings
    pub config: AppConfig,
    /// Current user session if authenticated
    pub current_session: Option<UserSession>,
}

/// Application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// List of configured Jellyfin servers
    pub servers: Vec<ServerConfig>,
    /// UI preferences and settings
    pub ui_settings: UiSettings,
}

/// Server configuration
#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub url: String,
}

/// User session information
#[derive(Clone, Debug)]
pub struct UserSession {
    pub server_id: String,
    pub user_id: String,
    pub username: String,
    pub access_token: String,
}

/// UI settings and preferences
#[derive(Clone, Debug)]
pub struct UiSettings {
    pub theme: String,
    pub window_size: (f32, f32),
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            ui_settings: UiSettings {
                theme: "dark".to_string(),
                window_size: (1024.0, 700.0),
            },
        }
    }
}

/// Main window component following GPUI patterns
struct MainWindow {
    /// Reference to the main app state
    app_state: Entity<JellyfinApp>,
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);

        div()
            .id("main-window")
            .size_full()
            .bg(rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .text_size(px(24.0))
                    .child("Crabfin - Jellyfin Client"),
            )
            .child(
                div()
                    .mt_4()
                    .text_color(rgb(0xcccccc))
                    .text_size(px(14.0))
                    .child(format!(
                        "Servers configured: {}",
                        app_state.config.servers.len()
                    )),
            )
            .child(
                div()
                    .mt_2()
                    .text_color(rgb(0xcccccc))
                    .text_size(px(14.0))
                    .child(match &app_state.current_session {
                        Some(session) => format!("Logged in as: {}", session.username),
                        None => "Not logged in".to_string(),
                    }),
            )
            .child(
                div()
                    .mt_4()
                    .text_color(rgb(0x888888))
                    .text_size(px(12.0))
                    .child("Error handling infrastructure initialized"),
            )
    }
}

impl JellyfinApp {
    /// Create a new JellyfinApp instance
    pub fn new() -> Self {
        info!("Initializing JellyfinApp");

        Self {
            config: AppConfig::default(),
            current_session: None,
        }
    }

    /// Add a server configuration
    pub fn add_server(&mut self, name: String, url: String) {
        let server = ServerConfig {
            id: format!("server_{}", self.config.servers.len()),
            name,
            url,
        };

        self.config.servers.push(server);
        info!("Added server configuration");
    }

    /// Set current user session
    pub fn set_session(&mut self, session: UserSession) {
        info!("Setting user session for user: {}", session.username);
        self.current_session = Some(session);
    }

    /// Handle application errors with user-friendly messaging
    pub fn handle_error(&self, error: anyhow::Error) -> String {
        let user_message = error.user_message();
        error!("Application error: {:#}", error);
        user_message
    }

    /// Validate server configuration with error handling
    pub fn validate_server_config(&self, url: &str) -> Result<()> {
        if url.is_empty() {
            return Err(crate::config_error!(
                crate::error::ConfigError::InvalidFormat("Server URL cannot be empty".to_string()),
                "Server configuration validation"
            ));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(crate::config_error!(
                crate::error::ConfigError::InvalidFormat("Server URL must start with http:// or https://".to_string()),
                "Server URL validation"
            ));
        }

        Ok(())
    }

    /// Clear current session with error handling
    pub fn clear_session(&mut self) -> Result<()> {
        match self.current_session.take() {
            Some(session) => {
                info!("Cleared session for user: {}", session.username);
                Ok(())
            }
            None => {
                let auth_error = crate::auth_error!(
                    crate::error::AuthError::AuthenticationRequired,
                    "Attempted to clear session when no session exists"
                );
                Err(auth_error)
            }
        }
    }

    /// Run the GPUI application
    pub async fn run() -> Result<()> {
        info!("Starting Jellyfin GPUI application");

        // Create the GPUI application
        Application::new().run(move |cx: &mut App| {
            debug!("GPUI Application context initialized");

            // Create the main app state
            let app_state = cx.new(|_| JellyfinApp::new());

            // Set up window bounds
            let bounds = Bounds::centered(None, size(px(1024.0), px(700.0)), cx);

            // Open the main window
            if let Err(e) = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_background: WindowBackgroundAppearance::Opaque,
                    window_decorations: Some(WindowDecorations::Client),
                    window_min_size: Some(size(px(800.0), px(600.0))),
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::from("Crabfin - Jellyfin Client")),
                        appears_transparent: false,
                        traffic_light_position: Some(Point {
                            x: px(12.0),
                            y: px(11.0),
                        }),
                    }),
                    app_id: Some("com.hitarashi.crabfin".to_string()),
                    kind: WindowKind::Normal,
                    ..Default::default()
                },
                move |window, cx| {
                    window.set_window_title("Crabfin - Jellyfin Client");

                    debug!("Main window created successfully");

                    // Create the main window component
                    cx.new(|_cx| MainWindow {
                        app_state: app_state.clone(),
                    })
                },
            ) {
                let ui_error = AppError::UserInterface(UiError::WindowCreationFailed(
                    format!("GPUI window creation failed: {:?}", e)
                ));
                let error_msg = ErrorHandler::handle_error(&anyhow::Error::new(ui_error));
                error!("Window creation failed: {}", error_msg);
                return;
            }

            info!("GPUI application started successfully");
        });

        Ok(())
    }
}
