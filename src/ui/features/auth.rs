use gpui::prelude::*;
use gpui::*;

use crate::ui::components::ui::button::{Button, ButtonVariant};
use crate::ui::components::ui::checkbox::Checkbox;
use crate::ui::components::ui::text_input::TextInput;
use crate::ui::theme::Theme;

pub struct AuthView {
    focus_handle: FocusHandle,
    server_name: SharedString,
    username_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    remember_me: bool,
    error_message: Option<String>,
    is_authenticating: bool,
    on_login: Option<Box<dyn Fn(String, String, bool, &mut Window, &mut Context<Self>) + 'static>>,
    on_back: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + 'static>>,
}

impl AuthView {
    pub fn new(server_name: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let username_input = cx.new(|cx| {
            TextInput::new(cx)
                .with_placeholder("Username")
        });

        let password_input = cx.new(|cx| {
            TextInput::new(cx)
                .with_placeholder("Password")
        });

        Self {
            focus_handle: cx.focus_handle(),
            server_name: server_name.into(),
            username_input,
            password_input,
            remember_me: true,  // Default to checked
            error_message: None,
            is_authenticating: false,
            on_login: None,
            on_back: None,
        }
    }

    pub fn set_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.error_message = Some(message);
        self.is_authenticating = false;
        cx.notify();
    }

    pub fn set_authenticating(&mut self, authenticating: bool, cx: &mut Context<Self>) {
        self.is_authenticating = authenticating;
        cx.notify();
    }

    pub fn on_login(mut self, handler: impl Fn(String, String, bool, &mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_login = Some(Box::new(handler));
        self
    }

    pub fn on_back(mut self, handler: impl Fn(&mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_back = Some(Box::new(handler));
        self
    }
}

impl Render for AuthView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("auth-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background())
            .text_color(theme.on_background())
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .font_family(theme.font_family())
                    .child(format!("Login to {}", self.server_name))
            )
            .child(
                div()
                    .w_full()
                    .max_w(px(400.0))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.username_input.clone())
                    .child(self.password_input.clone())
                    .child(
                        Checkbox::new("remember-me", "Remember me")
                            .checked(self.remember_me)
                            .on_change(cx.listener(|this, checked, _window, cx| {
                                this.remember_me = *checked;
                                cx.notify();
                            }))
                    )
            )
            .children(
                self.error_message.as_ref().map(|msg| {
                    div()
                        .text_color(theme.error())
                        .child(msg.clone())
                })
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        Button::new("Back")
                            .variant(ButtonVariant::Text)
                            .disabled(self.is_authenticating)
                            .on_click(cx.listener(|this, _, window, cx| {
                                if let Some(callback) = &this.on_back {
                                    callback(window, cx);
                                }
                            }))
                    )
                    .child(
                        Button::new(if self.is_authenticating { "Logging in..." } else { "Login" })
                            .variant(ButtonVariant::Filled)
                            .disabled(self.is_authenticating)
                            .on_click(cx.listener(|this, _, window, cx| {
                                if this.is_authenticating {
                                    return;
                                }
                                let username = this.username_input.read(cx).content.to_string();
                                let password = this.password_input.read(cx).content.to_string();

                                if username.is_empty() || password.is_empty() {
                                    this.error_message = Some("Username and password are required".to_string());
                                    cx.notify();
                                    return;
                                }

                                if let Some(callback) = &this.on_login {
                                    this.is_authenticating = true;
                                    this.error_message = None;
                                    cx.notify();
                                    callback(username, password, this.remember_me, window, cx);
                                }
                            }))
                    )
            )
    }
}

impl Focusable for AuthView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
