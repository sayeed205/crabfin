use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView, ConfigGlobal};
use crate::views::server_list::ServerListView;
use jellyfin::client::AuthenticatedClient;
use jellyfin::library::get_views;
use jellyfin::models::BaseItemDto;

pub struct LibraryView {
    focus_handle: FocusHandle,
    client: AuthenticatedClient,
    state: LibraryState,
}

enum LibraryState {
    Loading,
    Error(String),
    Loaded(Vec<BaseItemDto>),
}

impl LibraryView {
    pub fn new(client: AuthenticatedClient, cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|cx| {
            let mut view = Self {
                focus_handle,
                client,
                state: LibraryState::Loading,
            };
            view.fetch_libraries(cx);
            view
        })
    }

    fn fetch_libraries(&mut self, cx: &mut Context<Self>) {
        self.state = LibraryState::Loading;
        cx.notify();

        let client = self.client.clone();
        cx.spawn(move |view: WeakEntity<LibraryView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let result = crate::runtime::runtime()
                    .spawn(async move { get_views(&client).await })
                    .await;

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok(Ok(response)) => {
                            view.state = LibraryState::Loaded(response.items);
                        }
                        Ok(Err(e)) => {
                            view.state = LibraryState::Error(e.to_string());
                        }
                        Err(e) => {
                            view.state = LibraryState::Error(format!("Task failed: {}", e));
                        }
                    }
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn handle_logout(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let (server_id, user_id) = cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, _| {
                let s_id = model.settings.last_server_id;
                let u_id = model.settings.last_user_id.clone();

                model.settings.last_server_id = None;
                model.settings.last_user_id = None;
                let _ = model.save();

                (s_id, u_id)
            })
        });

        if let (Some(s_id), Some(u_id)) = (server_id, user_id) {
            if let Err(e) = credentials::delete_token(s_id, &u_id) {
                tracing::error!("Failed to delete token: {}", e);
            }
        }

        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::ServerList(ServerListView::new(cx));
                cx.notify();
            });
        });
    }
}

impl Render for LibraryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("Libraries"),
                    )
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .child("Logout")
                            .id("logout_btn")
                            .on_click(cx.listener(Self::handle_logout)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(match &self.state {
                        LibraryState::Loading => div().child("Loading libraries..."),
                        LibraryState::Error(msg) => {
                            div().child(msg.clone()).text_color(rgb(0xf38ba8))
                        }
                        LibraryState::Loaded(items) => {
                            let list = div().flex().flex_col().gap_2();
                            items.iter().fold(list, |list, item| {
                                list.child(
                                    div()
                                        .p_2()
                                        .bg(rgb(0x313244))
                                        .rounded_md()
                                        .child(item.name.clone())
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(0xa6adc8))
                                                .child(format!("{:?}", item.collection_type)),
                                        ),
                                )
                            })
                        }
                    }),
            )
    }
}

impl Focusable for LibraryView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
