use gpui::prelude::*;
use gpui::*;

use crate::models::UserSession;
use crate::ui::components::ui::button::{Button, ButtonVariant};
use crate::ui::theme::Theme;

pub struct LoggedInView {
    focus_handle: FocusHandle,
    session: UserSession,
    server_url: String,
    on_logout: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + 'static>>,
}

impl LoggedInView {
    pub fn new(session: UserSession, server_url: String, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            session,
            server_url,
            on_logout: None,
        }
    }

    pub fn on_logout(mut self, handler: impl Fn(&mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_logout = Some(Box::new(handler));
        self
    }
}

impl Render for LoggedInView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("logged-in-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background())
            .text_color(theme.on_background())
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .font_family(theme.font_family())
                    .child("Logged In Successfully!")
            )
            .child(
                div()
                    .w_full()
                    .max_w(px(500.0))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .p_4()
                            .bg(theme.surface())
                            .rounded(px(8.0))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Server Information")
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.on_surface_variant())
                                            .child("Name:")
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(self.session.server_name.clone())
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.on_surface_variant())
                                            .child("URL:")
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(self.server_url.clone())
                                    )
                            )
                    )
                    .child(
                        div()
                            .p_4()
                            .bg(theme.surface())
                            .rounded(px(8.0))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("User Information")
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.on_surface_variant())
                                            .child("Username:")
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(self.session.username.clone())
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.on_surface_variant())
                                            .child("User ID:")
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(self.session.user_id.clone())
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.on_surface_variant())
                                            .child("Role:")
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(if self.session.is_admin { "Administrator" } else { "User" })
                                    )
                            )
                    )
            )
            .child(
                Button::new("Logout")
                    .variant(ButtonVariant::Text)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(callback) = &this.on_logout {
                            callback(window, cx);
                        }
                    }))
            )
    }
}

impl Focusable for LoggedInView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
