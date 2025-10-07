use gpui::*;
use crate::auth::UserSession;

/// Main application window that contains all UI components
pub struct MainWindow {
    app_view: AppView,
}

impl MainWindow {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            app_view: AppView::new(),
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.app_view.render_content())
    }
}

/// Main application view that manages different screens
pub struct AppView {
    current_screen: AppScreen,
    user_session: Option<UserSession>,
}

impl AppView {
    pub fn new() -> Self {
        Self {
            current_screen: AppScreen::Login,
            user_session: None,
        }
    }

    pub fn set_session(&mut self, session: UserSession) {
        self.user_session = Some(session);
        self.current_screen = AppScreen::Home;
    }

    pub fn logout(&mut self) {
        self.user_session = None;
        self.current_screen = AppScreen::Login;
    }

    pub fn render_content(&mut self) -> Div {
        match self.current_screen {
            AppScreen::Login => self.render_login_screen(),
            AppScreen::Home => self.render_home_screen(),
            AppScreen::Library => self.render_library_screen(),
            AppScreen::Settings => self.render_settings_screen(),
        }
    }
}

impl AppView {
    fn render_login_screen(&mut self) -> Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_8()
                    .child("Jellyfin Login")
                    .child("Login screen will be implemented in later tasks")
            )
    }

    fn render_home_screen(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header())
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Home screen content will be implemented in later tasks")
            )
    }

    fn render_library_screen(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header())
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Library screen content will be implemented in later tasks")
            )
    }

    fn render_settings_screen(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header())
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Settings screen content will be implemented in later tasks")
            )
    }

    fn render_header(&mut self) -> Div {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .border_b_1()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child("Home")
                    .child("Library")
                    .child("Settings")
            )
            .child(
                if let Some(session) = &self.user_session {
                    div().child(format!("Welcome, {}", session.username))
                } else {
                    div().child("Not logged in")
                }
            )
    }
}

/// Different screens/views in the application
#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Login,
    Home,
    Library,
    Settings,
}

/// Base trait for UI components
pub trait Component {
    fn update(&mut self);
}

/// UI state management
#[derive(Debug, Clone)]
pub struct UiState {
    pub loading: bool,
    pub error_message: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            loading: false,
            error_message: None,
        }
    }
}

impl UiState {
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
        if loading {
            self.error_message = None;
        }
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.loading = false;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}