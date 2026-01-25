use crate::prelude::*;
use crate::views::login::LoginView;
use crate::views::user_selection::UserSelectionView;

pub struct ServerListView {
    focus_handle: FocusHandle,
    selection_mode: bool,
}

impl ServerListView {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self {
            focus_handle,
            selection_mode: false,
        })
    }

    pub fn new_selection_mode(cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self {
            focus_handle,
            selection_mode: true,
        })
    }

    fn add_server(_: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                let view = AddServerView::new(cx);
                state.current_view = AppView::AddServer(view);
                cx.notify();
            });
        });
    }

    fn connect_to_server(server: settings::ServerConfig, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                let view = LoginView::new(server, cx);
                state.current_view = AppView::Login(view);
                cx.notify();
            });
        });
    }

    fn select_server(server: settings::ServerConfig, cx: &mut App) {
        cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, _| {
                model.settings.default_server_id = Some(server.id);
                let _ = model.save();
            });
        });

        let config_global = cx.global::<ConfigGlobal>();
        let config = config_global.model.read(cx);
        let users = config.get_users_for_server(server.id);

        let server_clone = server.clone();
        let users_clone = users.into_iter().cloned().collect::<Vec<_>>();
        let has_users = !users_clone.is_empty();

        cx.update_global::<AppStateGlobal, _>(move |global, cx| {
            global.0.update(cx, |state, cx| {
                if has_users {
                    state.current_view = AppView::UserSelection(UserSelectionView::new(
                        server_clone,
                        users_clone,
                        cx,
                    ));
                } else {
                    state.current_view = AppView::Login(LoginView::new(server_clone, cx));
                }
                cx.notify();
            });
        });
    }

    fn remove_server(id: uuid::Uuid, cx: &mut App) {
        cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, _cx| {
                model.remove_server(id);
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
        let selection_mode = self.selection_mode;

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
                            .child(if selection_mode {
                                "Select Default Server"
                            } else {
                                "Servers"
                            }),
                    )
                    .child(if !selection_mode {
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Primary)
                            .child("Add Server")
                            .id("add-server-btn")
                            .on_click(|ev, win, cx| Self::add_server(ev, win, cx))
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }),
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
                                let server_for_connect = server.clone();
                                let server_for_select = server.clone();
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
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                button()
                                                    .size(ButtonSize::Regular)
                                                    .intent(ButtonIntent::Primary)
                                                    .child(if selection_mode {
                                                        "Select"
                                                    } else {
                                                        "Connect"
                                                    })
                                                    .id(SharedString::from(format!(
                                                        "connect-{}",
                                                        server.id
                                                    )))
                                                    .on_click(move |_, _, cx| {
                                                        if selection_mode {
                                                            Self::select_server(
                                                                server_for_select.clone(),
                                                                cx,
                                                            );
                                                        } else {
                                                            Self::connect_to_server(
                                                                server_for_connect.clone(),
                                                                cx,
                                                            );
                                                        }
                                                    }),
                                            )
                                            .child(if !selection_mode {
                                                button()
                                                    .size(ButtonSize::Regular)
                                                    .intent(ButtonIntent::Danger)
                                                    .child("Remove")
                                                    .id(SharedString::from(format!(
                                                        "remove-{}",
                                                        server.id
                                                    )))
                                                    .on_click(move |_, _, cx| {
                                                        Self::remove_server(server.id, cx);
                                                    })
                                                    .into_any_element()
                                            } else {
                                                div().into_any_element()
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
