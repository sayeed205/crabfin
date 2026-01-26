use crate::components::button::{button, ButtonIntent, ButtonSize};
use crate::components::episode_card::episode_card;
use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView};
use crate::views::item_detail::ItemDetailView;
use gpui::Styled;
use jellyfin::client::AuthenticatedClient;
use jellyfin::library::{get_episodes, EpisodesQuery};
use jellyfin::models::BaseItemDto;

pub struct SeasonView {
    focus_handle: FocusHandle,
    client: AuthenticatedClient,
    series_id: String,
    season_id: String,
    series_name: String,
    season_name: String,
    username: String,
    episodes: Vec<BaseItemDto>,
    loading: bool,
}

impl SeasonView {
    pub fn new(
        client: AuthenticatedClient,
        series_id: String,
        season_id: String,
        series_name: String,
        season_name: String,
        username: String,
        cx: &mut App,
    ) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|cx| {
            let mut view = Self {
                focus_handle,
                client,
                series_id,
                season_id,
                series_name,
                season_name,
                username,
                episodes: Vec::new(),
                loading: true,
            };
            view.fetch_episodes(cx);
            view
        })
    }

    fn fetch_episodes(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();

        let client = self.client.clone();
        let series_id = self.series_id.clone();
        let season_id = self.season_id.clone();

        cx.spawn(move |view: WeakEntity<SeasonView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let query = EpisodesQuery::default()
                    .with_season_id(season_id)
                    .with_user_id(client.user_id());
                let result = crate::runtime::runtime()
                    .spawn(async move { get_episodes(&client, &series_id, &query).await })
                    .await;

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok(Ok(response)) => {
                            view.episodes = response.items;
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Failed to fetch episodes: {}", e);
                        }
                        Err(e) => {
                            tracing::warn!("Task failed fetching episodes: {}", e);
                        }
                    }
                    view.loading = false;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn handle_back(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let series_id = self.series_id.clone();
        let username = self.username.clone();
        cx.update_global::<AppStateGlobal, _>(move |global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::ItemDetail(ItemDetailView::new(
                    client, series_id, username, cx,
                ));
                cx.notify();
            });
        });
    }
}

impl Render for SeasonView {
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
                    .items_center()
                    .p_4()
                    .gap_4()
                    .border_b_1()
                    .border_color(rgb(0x313244))
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .id("back-btn")
                            .child("Back")
                            .on_click(cx.listener(Self::handle_back)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child(self.series_name.clone())
                            .child(div().text_color(rgb(0x6c7086)).child(">"))
                            .child(self.season_name.clone()),
                    ),
            )
            .child(
                div()
                    .id("season-scroll")
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .overflow_y_scroll()
                    .child(if self.loading {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_32()
                            .child("Loading episodes...")
                            .into_any_element()
                    } else if self.episodes.is_empty() {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_32()
                            .child("No episodes found")
                            .into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(self.episodes.iter().map(episode_card))
                            .into_any_element()
                    }),
            )
    }
}

impl Focusable for SeasonView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
