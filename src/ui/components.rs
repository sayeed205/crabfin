//! GPUI UI components
//!
//! This module contains reusable UI components built with GPUI.

use gpui::*;

use super::theme::Theme;

/// Main application window component
pub struct MainWindow {
    /// Window focus state
    focus_handle: FocusHandle,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("main-window")
            .key_context("main-window")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background_primary)
            .text_color(theme.text_primary)
            .flex()
            .flex_col()
            .child(
                // Header/title bar area
                div()
                    .id("header")
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
                            .child("Crabfin - Jellyfin Client")
                    )
            )
            .child(
                // Main content area
                div()
                    .id("content")
                    .flex_1()
                    .w_full()
                    .bg(theme.background_primary)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xl()
                            .text_color(theme.text_secondary)
                            .child("Welcome to Crabfin")
                    )
            )
            .child(
                // Status bar
                div()
                    .id("status-bar")
                    .w_full()
                    .h(px(24.0))
                    .bg(theme.background_secondary)
                    .border_t_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .px_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .child("Ready")
                    )
            )
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}