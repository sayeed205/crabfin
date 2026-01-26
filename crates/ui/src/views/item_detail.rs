use crate::components::button::{button, ButtonIntent, ButtonSize};
use crate::components::item_card::item_card;
use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView};
use crate::views::home::HomeView;
use crate::views::season::SeasonView;
use gpui::Styled;
use jellyfin::client::AuthenticatedClient;
use jellyfin::library::{get_item, get_seasons, get_similar, SeasonsQuery, SimilarQuery};
use jellyfin::models::BaseItemDto;

pub struct ItemDetailView {
    focus_handle: FocusHandle,
    client: AuthenticatedClient,
    item_id: String,
    username: String,
    item: Option<BaseItemDto>,
    seasons: Vec<BaseItemDto>,
    similar: Vec<BaseItemDto>,
    loading: bool,
}

impl ItemDetailView {
    pub fn new(
        client: AuthenticatedClient,
        item_id: String,
        username: String,
        cx: &mut App,
    ) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|cx| {
            let mut view = Self {
                focus_handle,
                client,
                item_id,
                username,
                item: None,
                seasons: Vec::new(),
                similar: Vec::new(),
                loading: true,
            };
            view.fetch_data(cx);
            view
        })
    }

    fn fetch_data(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();

        let client = self.client.clone();
        let item_id = self.item_id.clone();

        cx.spawn(move |view: WeakEntity<ItemDetailView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let client_clone1 = client.clone();
                let item_id_clone1 = item_id.clone();
                let item_result = crate::runtime::runtime()
                    .spawn(async move { get_item(&client_clone1, &item_id_clone1).await })
                    .await;

                let client_clone2 = client.clone();
                let item_id_clone2 = item_id.clone();
                let seasons_result = crate::runtime::runtime()
                    .spawn(async move {
                        let query = SeasonsQuery::default().with_user_id(client_clone2.user_id());
                        get_seasons(&client_clone2, &item_id_clone2, &query).await
                    })
                    .await;

                let client_clone3 = client.clone();
                let item_id_clone3 = item_id.clone();
                let similar_result = crate::runtime::runtime()
                    .spawn(async move {
                        let query = SimilarQuery::default()
                            .with_user_id(client_clone3.user_id())
                            .with_limit(10);
                        get_similar(&client_clone3, &item_id_clone3, &query).await
                    })
                    .await;

                view.update(&mut cx, |view, cx| {
                    if let Ok(Ok(item)) = item_result {
                        view.item = Some(item);
                    }
                    if let Ok(Ok(seasons)) = seasons_result {
                        view.seasons = seasons.items;
                    }
                    if let Ok(Ok(similar)) = similar_result {
                        view.similar = similar.items;
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
        let username = self.username.clone();
        cx.update_global::<AppStateGlobal, _>(move |global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::Home(HomeView::new(username, client, cx));
                cx.notify();
            });
        });
    }

    fn handle_season_click(
        &mut self,
        season_id: String,
        season_name: String,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let client = self.client.clone();
        let series_id = self.item_id.clone();
        let username = self.username.clone();
        let series_name = self.item.as_ref().and_then(|i| i.name.clone()).unwrap_or_default();
        
        cx.update_global::<AppStateGlobal, _>(move |global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::Season(SeasonView::new(
                    client, series_id, season_id, series_name, season_name, username, cx,
                ));
                cx.notify();
            });
        });
    }

    fn render_season_card(&self, season: &BaseItemDto, cx: &mut Context<Self>) -> impl IntoElement {
        let id = season.id.clone();
        let name = season.name.clone().unwrap_or_default();
        let name_clone = name.clone();
        let index = season.index_number.unwrap_or(0);

        div()
            .id(SharedString::from(id.clone()))
            .w_48()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .bg(rgb(0x313244))
            .rounded_md()
            .hover(|s| s.bg(rgb(0x45475a)))
            .cursor_pointer()
            .on_click(cx.listener(move |view, event, window, cx| {
                view.handle_season_click(id.clone(), name_clone.clone(), event, window, cx)
            }))
            .child(
                div()
                    .h_64()
                    .bg(rgb(0x181825))
                    .rounded_sm()
                    .flex()
                    .justify_center()
                    .items_center()
                    .text_color(rgb(0x585b70))
                    .child(format!("Season {}", index)),
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_sm()
                    .child(name),
            )
    }
}

impl Render for ItemDetailView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.loading {
            return div()
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .bg(rgb(0x1e1e2e))
                .text_color(rgb(0xcdd6f4))
                .child("Loading...")
                .into_any_element();
        }

        let item = match &self.item {
            Some(i) => i,
            None => {
                return div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .bg(rgb(0x1e1e2e))
                    .text_color(rgb(0xcdd6f4))
                    .child("Item not found")
                    .into_any_element()
            }
        };

        let mut season_cards = Vec::new();
        for s in &self.seasons {
            season_cards.push(self.render_season_card(s, cx));
        }

        let mut similar_cards = Vec::new();
        for item in &self.similar {
            similar_cards.push(item_card(item, &self.client, self.username.clone(), cx));
        }

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
                    .border_b_1()
                    .border_color(rgb(0x313244))
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .id("back-btn")
                            .child("Back")
                            .on_click(cx.listener(Self::handle_back)),
                    ),
            )
            .child(
                div()
                    .id("detail-scroll")
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_6()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .gap_6()
                            .child(
                                div()
                                    .w_64()
                                    .h_96()
                                    .bg(rgb(0x181825))
                                    .rounded_lg()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .text_color(rgb(0x585b70))
                                    .child("No Image"),
                            )
                            .child(
                                div()
                                    .id("detail-content")
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_3xl()
                                            .font_weight(FontWeight::BOLD)
                                            .child(item.name.clone().unwrap_or_default()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
                                            .text_color(rgb(0xa6adc8))
                                            .child(
                                                item.production_year
                                                    .map(|y| y.to_string())
                                                    .unwrap_or_default(),
                                            )
                                            .child(item.official_rating.clone().unwrap_or_default())
                                            .child(format!(
                                                "{} min",
                                                item.run_time_ticks.unwrap_or(0) / 10_000 / 1000 / 60
                                            )),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .children(item.genres.clone().unwrap_or_default().iter().map(
                                                |g| {
                                                    div()
                                                        .px_2()
                                                        .py_1()
                                                        .bg(rgb(0x313244))
                                                        .rounded_md()
                                                        .text_xs()
                                                        .child(g.clone())
                                                },
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_lg()
                                            .child(item.overview.clone().unwrap_or_default()),
                                    ),
                            ),
                    ),
            )
            .child(if !self.seasons.is_empty() {
                div()
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Seasons"),
                    )
                    .child(
                        div()
                            .id("seasons-scroll")
                            .flex()
                            .gap_4()
                            .overflow_x_scroll()
                            .pb_2()
                            .children(season_cards),
                    )
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .child(if !self.similar.is_empty() {
                div()
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Similar Items"),
                    )
                    .child(
                        div()
                            .id("similar-scroll")
                            .flex()
                            .gap_4()
                            .overflow_x_scroll()
                            .pb_2()
                            .children(similar_cards),
                    )
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .into_any_element()
    }
}

impl Focusable for ItemDetailView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
