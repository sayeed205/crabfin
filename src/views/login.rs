use crate::components::PasswordInput;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, input::{Input, InputState}, *};

pub struct LoginView {
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    server_url: String,
    on_login: Box<dyn Fn(String, String, &mut Window, &mut Context<LoginView>) + 'static>,
    on_back: Box<dyn Fn(&mut Window, &mut Context<LoginView>) + 'static>,
    is_loading: bool,
    error_message: Option<String>,
    is_password_visible: bool,
}

impl LoginView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        server_url: String,
        on_login: impl Fn(String, String, &mut Window, &mut Context<LoginView>) + 'static,
        on_back: impl Fn(&mut Window, &mut Context<LoginView>) + 'static,
    ) -> Self {
        let username_input = cx.new(|cx| InputState::new(window, cx).placeholder("Username"));
        let password_input = cx.new(|cx| InputState::new(window, cx).placeholder("Password").masked(true));

        Self {
            username_input,
            password_input,
            server_url,
            on_login: Box::new(on_login),
            on_back: Box::new(on_back),
            is_loading: false,
            error_message: None,
            is_password_visible: false,
        }
    }

    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.is_loading = loading;
        cx.notify();
    }

    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error_message = error;
        cx.notify();
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_masked = !self.is_password_visible;

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.background)
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().text_xl().font_bold().child("Login"))
                    .child(div().text_sm().text_color(theme.muted_foreground).child(self.server_url.clone()))
                    .child(Input::new(&self.username_input).disabled(self.is_loading))
                    .child(
                        PasswordInput::new(&self.password_input, is_masked)
                            .on_toggle(cx.listener(|this, _, window, cx| {
                                this.is_password_visible = !this.is_password_visible;
                                let is_masked = !this.is_password_visible;

                                // We need to update the InputState.
                                // set_masked requires &mut Window.
                                // We capture window from the listener scope.
                                // Note: This might conflict with cx borrow if update borrows something that overlaps with window.
                                // But in GPUI 0.2, Window and App (Context) are separate.
                                this.password_input.update(cx, |state, cx| {
                                    state.set_masked(is_masked, window, cx);
                                });
                                cx.notify();
                            }))
                    )
                    .children(self.error_message.as_ref().map(|msg| {
                        div().text_sm().text_color(theme.danger).child(msg.clone())
                    }))
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("back")
                                    .label("Back")
                                    .disabled(self.is_loading)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_back)(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("login")
                                    .primary()
                                    .label(if self.is_loading { "Logging in..." } else { "Login" })
                                    .disabled(self.is_loading)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        let username = this.username_input.read(cx).value();
                                        let password = this.password_input.read(cx).value();
                                        if !username.is_empty() {
                                            (this.on_login)(username.to_string(), password.to_string(), window, cx);
                                        }
                                    }))
                            )
                    )
            )
    }
}
