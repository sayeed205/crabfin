use crate::components::item_card::item_card;
use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView};
use crate::views::library::LibraryView;
use jellyfin::client::AuthenticatedClient;
use jellyfin::library::{get_items, ItemsQuery};
use jellyfin::models::BaseItemDto;

pub struct ItemGridView {
    focus_handle: FocusHandle,
    library_name: String,
    library_id: String,
    items: Vec<BaseItemDto>,
    loading: bool,
    error: Option<String>,
    client: AuthenticatedClient,
    username: String,
}

impl ItemGridView {
    pub fn new(
        library_id: String,
        library_name: String,
        client: AuthenticatedClient,
        username: String,
        cx: &mut App,
    ) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|cx| {
            let mut view = Self {
                focus_handle,
                library_name,
                library_id,
                items: Vec::new(),
                loading: true,
                error: None,
                client,
                username,
            };
            view.fetch_items(cx);
            view
        })
    }

    fn fetch_items(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        cx.notify();

        let client = self.client.clone();
        let library_id = self.library_id.clone();

        cx.spawn(move |view: WeakEntity<ItemGridView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let query = ItemsQuery::default()
                    .with_parent_id(library_id)
                    .with_sort_by("SortName")
                    .with_limit(100);

                let result = crate::runtime::runtime()
                    .spawn(async move { get_items(&client, &query).await })
                    .await;

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok(Ok(response)) => {
                            view.items = response.items;
                            view.loading = false;
                        }
                        Ok(Err(e)) => {
                            view.error = Some(e.to_string());
                            view.loading = false;
                        }
                        Err(e) => {
                            view.error = Some(format!("Task failed: {}", e));
                            view.loading = false;
                        }
                    }
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
                state.current_view = AppView::Library(LibraryView::new(client, username, cx));
                cx.notify();
            });
        });
    }
}

impl Render for ItemGridView {
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
                            .id("back-btn")
                            .child("Back")
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .on_click(cx.listener(Self::handle_back)),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child(self.library_name.clone()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .size_full()
                    .child(if self.loading {
                        div()
                            .flex()
                            .justify_center()
                            .items_center()
                            .size_full()
                            .child("Loading items...")
                    } else if let Some(error) = &self.error {
                        div()
                            .flex()
                            .justify_center()
                            .items_center()
                            .size_full()
                            .text_color(rgb(0xf38ba8))
                            .child(error.clone())
                    } else {
                        let mut cards = Vec::new();
                        for item in &self.items {
                            cards.push(item_card(
                                item,
                                self.client.clone(),
                                self.username.clone(),
                                cx,
                            ));
                        }

                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .children(cards)
                    }),
            )
    }
}

impl Focusable for ItemGridView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
