//! GPUI views
//!
//! This module contains main application views and screens.

use gpui::*;
use std::sync::Arc;
use tracing::{debug, info};

use super::theme::Theme;
use crate::auth::{AuthManager, SessionManager, UserSession};

/// Main application view that manages the overall UI state
pub struct MainView {
    focus_handle: FocusHandle,
    current_view: ViewState,
    auth_manager: Arc<tokio::sync::Mutex<SessionManager>>,
}

/// Different view states of the application
#[derive(Debug, Clone, PartialEq)]
pub enum ViewState {
    /// Server selection/connection view
    ServerSelection,
    /// User authentication view
    Authentication { server_id: String },
    /// Main media library view
    Library { session: UserSession },
    /// Settings view
    Settings,
    /// Loading state
    Loading { message: String },
    /// Error state
    Error { message: String },
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        debug!("Creating main view");

        Self {
            focus_handle: cx.focus_handle(),
            current_view: ViewState::ServerSelection,
            auth_manager: Arc::new(tokio::sync::Mutex::new(SessionManager::new())),
        }
    }

    /// Switch to a different view state
    pub fn switch_view(&mut self, new_view: ViewState, cx: &mut Context<Self>) {
        info!("Switching view from {:?} to {:?}", self.current_view, new_view);
        self.current_view = new_view;
        cx.notify();
    }

    /// Get the current view state
    pub fn current_view(&self) -> &ViewState {
        &self.current_view
    }

    /// Get the auth manager
    pub fn auth_manager(&self) -> Arc<tokio::sync::Mutex<SessionManager>> {
        self.auth_manager.clone()
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("main-view")
            .key_context("main-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background_primary)
            .text_color(theme.text_primary)
            .flex()
            .flex_col()
            .child(self.render_current_view(cx))
    }
}

impl MainView {
    fn render_current_view(&mut self, cx: &mut Context<Self>) -> AnyElement {
        match &self.current_view {
            ViewState::ServerSelection => self.render_server_selection(cx).into_any_element(),
            ViewState::Authentication { server_id } => self.render_authentication(server_id.clone(), cx).into_any_element(),
            ViewState::Library { session } => self.render_library(session.clone(), cx).into_any_element(),
            ViewState::Settings => self.render_settings(cx).into_any_element(),
            ViewState::Loading { message } => self.render_loading(message.clone(), cx).into_any_element(),
            ViewState::Error { message } => self.render_error(message.clone(), cx).into_any_element(),
        }
    }

    fn render_server_selection(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("server-selection")
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child("Connect to Jellyfin Server")
            )
            .child(
                div()
                    .text_base()
                    .text_color(theme.text_secondary)
                    .child("Enter your server details to get started")
            )
            .child(
                div()
                    .w(px(400.0))
                    .p_6()
                    .bg(theme.background_secondary)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_lg()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_secondary)
                            .child("Server URL")
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .bg(theme.background_primary)
                            .border_1()
                            .border_color(theme.border)
                            .rounded_lg()
                            .px_3()
                            .flex()
                            .items_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text_secondary)
                                    .child("http://your-server:8096")
                            )
                    )
            )
    }

    fn render_authentication(&mut self, _server_id: String, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("authentication")
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child("Sign In")
            )
            .child(
                div()
                    .text_base()
                    .text_color(theme.text_secondary)
                    .child("Enter your credentials to access your media")
            )
    }

    fn render_library(&mut self, _session: UserSession, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("library")
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .w_full()
                    .h(px(60.0))
                    .bg(theme.background_secondary)
                    .border_b_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .px_4()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child("Media Library")
                    )
            )
            .child(
                // Content
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xl()
                            .text_color(theme.text_secondary)
                            .child("Your media library will appear here")
                    )
            )
    }

    fn render_settings(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("settings")
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_xl()
                    .text_color(theme.text_secondary)
                    .child("Settings")
            )
    }

    fn render_loading(&mut self, message: String, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("loading")
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .text_color(theme.text_primary)
                    .child("Loading...")
            )
            .child(
                div()
                    .text_base()
                    .text_color(theme.text_secondary)
                    .child(message)
            )
    }

    fn render_error(&mut self, message: String, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("error")
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .text_color(theme.error)
                    .child("Error")
            )
            .child(
                div()
                    .text_base()
                    .text_color(theme.text_secondary)
                    .child(message)
            )
    }
}

impl Focusable for MainView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}