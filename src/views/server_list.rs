use crate::config::Server;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, *};

pub struct ServerListView {
    servers: Vec<Server>,
    on_select: Box<dyn Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static>,
    on_add: Box<dyn Fn(&mut Window, &mut Context<ServerListView>) + 'static>,
    on_delete: Box<dyn Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static>,
}

impl ServerListView {
    pub fn new(
        servers: Vec<Server>,
        on_select: impl Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static,
        on_add: impl Fn(&mut Window, &mut Context<ServerListView>) + 'static,
        on_delete: impl Fn(&Server, &mut Window, &mut Context<ServerListView>) + 'static,
    ) -> Self {
        Self {
            servers,
            on_select: Box::new(on_select),
            on_add: Box::new(on_add),
            on_delete: Box::new(on_delete),
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
                        let server_delete = server.clone();
                        div()
                            .id(i)
                            .p_4()
                            .border_1()
                            .border_color(theme.border)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.list_hover))
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_bold().child(server.name.clone()))
                                    .child(div().text_sm().text_color(theme.muted_foreground).child(server.url.clone()))
                            )
                            .child(
                                Button::new("delete")
                                    .icon(IconName::Close)
                                    .ghost()
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        (this.on_delete)(&server_delete, window, cx);
                                    }))
                            )
                            .on_click(cx.listener(move |this, _, window, cx| {
                                (this.on_select)(&server_clone, window, cx);
                            }))
                    }))
            )
    }
}
