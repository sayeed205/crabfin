use crate::prelude::*;

pub struct ServerListView {
    focus_handle: FocusHandle,
}

impl ServerListView {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self { focus_handle })
    }

    fn add_server(click: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                let view = AddServerView::new(cx);
                state.current_view = AppView::AddServer(view);
                cx.notify();
            });
        });
    }

    fn remove_server(id: uuid::Uuid, cx: &mut App) {
        cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, cx| {
                model.remove_server(id);
                // We should save here, but save is fallible. For now just unwrap or ignore error?
                // Ideally propagate error or show toast.
                let _ = model.save();
            });
        });
    }
}

impl Render for ServerListView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config_global = cx.global::<ConfigGlobal>();
        let config = config_global.model.read(cx);
        let servers = config.servers.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child("Servers"),
                    )
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Primary)
                            .child("Add Server")
                            .id("add-server-btn")
                            .on_click(|ev, win, cx| Self::add_server(ev, win, cx)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .children(if servers.is_empty() {
                        vec![div()
                            .flex()
                            .justify_center()
                            .p_8()
                            .text_color(rgb(0xa6adc8))
                            .child("No servers configured")
                            .into_any_element()]
                    } else {
                        servers
                            .into_iter()
                            .map(|server| {
                                div()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .p_4()
                                    .bg(rgb(0x313244))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .font_weight(FontWeight::BOLD)
                                                    .child(server.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0xa6adc8))
                                                    .child(server.url.clone()),
                                            ),
                                    )
                                    .child(
                                        button()
                                            .size(ButtonSize::Regular)
                                            .intent(ButtonIntent::Danger)
                                            .child("Remove")
                                            .id(SharedString::from(format!("remove-{}", server.id)))
                                            .on_click(move |_, _, cx| {
                                                Self::remove_server(server.id, cx);
                                            }),
                                    )
                                    .into_any_element()
                            })
                            .collect()
                    }),
            )
    }
}

impl Focusable for ServerListView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
