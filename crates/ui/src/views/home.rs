use crate::components::item_card::item_card;
use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView, ConfigGlobal};
use crate::views::library::LibraryView;
use crate::views::server_list::ServerListView;
use jellyfin::client::AuthenticatedClient;
use jellyfin::library::{get_latest, get_resume, LatestQuery, ResumeQuery};
use jellyfin::models::BaseItemDto;

pub struct HomeView {
    focus_handle: FocusHandle,
    username: String,
    client: AuthenticatedClient,
    resume_items: Vec<BaseItemDto>,
    latest_items: Vec<BaseItemDto>,
    loading_resume: bool,
    loading_latest: bool,
}

impl HomeView {
    pub fn new(username: String, client: AuthenticatedClient, cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|cx| {
            let mut view = Self {
                focus_handle,
                username,
                client,
                resume_items: Vec::new(),
                latest_items: Vec::new(),
                loading_resume: true,
                loading_latest: true,
            };
            view.fetch_resume(cx);
            view.fetch_latest(cx);
            view
        })
    }

    fn fetch_resume(&mut self, cx: &mut Context<Self>) {
        self.loading_resume = true;
        cx.notify();

        let client = self.client.clone();
        cx.spawn(move |view: WeakEntity<HomeView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let query = ResumeQuery::default().with_limit(20);
                let result = crate::runtime::runtime()
                    .spawn(async move { get_resume(&client, &query).await })
                    .await;

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok(Ok(response)) => {
                            view.resume_items = response.items;
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Failed to fetch resume items: {}", e);
                        }
                        Err(e) => {
                            tracing::warn!("Task failed fetching resume: {}", e);
                        }
                    }
                    view.loading_resume = false;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn fetch_latest(&mut self, cx: &mut Context<Self>) {
        self.loading_latest = true;
        cx.notify();

        let client = self.client.clone();
        cx.spawn(move |view: WeakEntity<HomeView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let query = LatestQuery::default().with_limit(20);
                let result = crate::runtime::runtime()
                    .spawn(async move { get_latest(&client, &query).await })
                    .await;

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok(Ok(items)) => {
                            view.latest_items = items;
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Failed to fetch latest items: {}", e);
                        }
                        Err(e) => {
                            tracing::warn!("Task failed fetching latest: {}", e);
                        }
                    }
                    view.loading_latest = false;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn handle_browse(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let client = self.client.clone();
        cx.update_global::<AppStateGlobal, _>(move |global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::Library(LibraryView::new(client, cx));
                cx.notify();
            });
        });
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
            credentials::delete_token(s_id, &u_id, cx).detach_and_log_err(cx);
        }

        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::ServerList(ServerListView::new(cx));
                cx.notify();
            });
        });
    }

    fn render_section(
        &self,
        id: &str,
        title: &str,
        items: &[BaseItemDto],
        loading: bool,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title.to_string()),
            )
            .child(if loading {
                div()
                    .h_72()
                    .flex()
                    .items_center()
                    .text_color(rgb(0xa6adc8))
                    .child("Loading...")
                    .into_any_element()
            } else if items.is_empty() {
                div()
                    .h_72()
                    .flex()
                    .items_center()
                    .text_color(rgb(0x6c7086))
                    .child("No items")
                    .into_any_element()
            } else {
                div()
                    .id(SharedString::from(id.to_string()))
                    .flex()
                    .gap_4()
                    .overflow_x_scroll()
                    .pb_2()
                    .children(items.iter().map(item_card))
                    .into_any_element()
            })
    }
}

impl Render for HomeView {
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
                    .border_b_1()
                    .border_color(rgb(0x313244))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child(format!("Welcome, {}!", self.username)),
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
                                    .child("Browse Libraries")
                                    .id("browse-btn")
                                    .on_click(cx.listener(Self::handle_browse)),
                            )
                            .child(
                                button()
                                    .size(ButtonSize::Regular)
                                    .intent(ButtonIntent::Secondary)
                                    .child("Logout")
                                    .id("logout-btn")
                                    .on_click(cx.listener(Self::handle_logout)),
                            ),
                    ),
            )
            .child(
                div()
                    .id("home-content")
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_6()
                    .overflow_y_scroll()
                    .size_full()
                    .child(self.render_section(
                        "resume-section",
                        "Continue Watching",
                        &self.resume_items,
                        self.loading_resume,
                    ))
                    .child(self.render_section(
                        "latest-section",
                        "Latest",
                        &self.latest_items,
                        self.loading_latest,
                    )),
            )
    }
}

impl Focusable for HomeView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
