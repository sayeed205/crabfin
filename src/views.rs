use crate::config::Server;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, input::{Input, InputState}, *};

pub struct ServerListView {
    servers: Vec<Server>,
    on_select: Box<dyn Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static>,
    on_add: Box<dyn Fn(&mut Window, &mut Context<ServerListView>) + 'static>,
}

impl ServerListView {
    pub fn new(
        servers: Vec<Server>,
        on_select: impl Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static,
        on_add: impl Fn(&mut Window, &mut Context<ServerListView>) + 'static,
    ) -> Self {
        Self {
            servers,
            on_select: Box::new(on_select),
            on_add: Box::new(on_add),
        }
    }
}

impl Render for ServerListView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

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
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(div().text_xl().font_bold().child("Select Server"))
                            .child(
                                Button::new("add_server")
                                    .primary()
                                    .label("Add Server")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_add)(window, cx);
                                    }))
                            )
                    )
                    .children(self.servers.iter().enumerate().map(|(i, server)| {
                        let server_clone = server.clone();
                        div()
                            .id(i)
                            .p_4()
                            .border_1()
                            .border_color(theme.border)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.list_hover))
                            .child(div().font_bold().child(server.name.clone()))
                            .child(div().text_sm().text_color(theme.muted_foreground).child(server.url.clone()))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                (this.on_select)(&server_clone, window, cx);
                            }))
                    }))
            )
    }
}

pub struct AddServerView {
    input: Entity<InputState>,
    on_connect: Box<dyn Fn(String, &mut Window, &mut Context<AddServerView>) + 'static>,
    on_cancel: Box<dyn Fn(&mut Window, &mut Context<AddServerView>) + 'static>,
}

impl AddServerView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        on_connect: impl Fn(String, &mut Window, &mut Context<AddServerView>) + 'static,
        on_cancel: impl Fn(&mut Window, &mut Context<AddServerView>) + 'static,
    ) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("https://jellyfin.example.com"));
        Self {
            input,
            on_connect: Box::new(on_connect),
            on_cancel: Box::new(on_cancel),
        }
    }
}

impl Render for AddServerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

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
                    .child(div().text_xl().font_bold().child("Add Server"))
                    .child(Input::new(&self.input))
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_cancel)(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("connect")
                                    .primary()
                                    .label("Connect")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        let url = this.input.read(cx).value();
                                        if !url.is_empty() {
                                            (this.on_connect)(url.to_string(), window, cx);
                                        }
                                    }))
                            )
                    )
            )
    }
}

pub struct LoginView {
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    server_url: String,
    on_login: Box<dyn Fn(String, String, &mut Window, &mut Context<LoginView>) + 'static>,
    on_back: Box<dyn Fn(&mut Window, &mut Context<LoginView>) + 'static>,
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
        }
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

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
                    .child(Input::new(&self.username_input))
                    .child(Input::new(&self.password_input))
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("back")
                                    .label("Back")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_back)(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("login")
                                    .primary()
                                    .label("Login")
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
