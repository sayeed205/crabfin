use crate::components::image_display::image_display;
use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView};
use crate::views::item_detail::ItemDetailView;
use jellyfin::client::{AuthenticatedClient, ImageType};
use jellyfin::models::BaseItemDto;
use std::hash::{Hash, Hasher};

pub fn item_card(
    item: &BaseItemDto,
    client: AuthenticatedClient,
    username: String,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let item_id = item.id.clone();
    let client_clone = client.clone();
    let username_clone = username.clone();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    item.id.hash(&mut hasher);
    let id_hash = hasher.finish();

    let image = if let Some(tag) = item.image_tags.as_ref().and_then(|t| t.get("Primary")) {
        image_display(
            item.id.clone(),
            ImageType::Primary,
            tag.clone(),
            192,
            None,
            client.clone(),
            cx,
        )
        .into_any_element()
    } else {
        div()
            .size_full()
            .bg(rgb(0x181825))
            .flex()
            .justify_center()
            .items_center()
            .text_color(rgb(0x585b70))
            .child("No Image")
            .into_any_element()
    };

    div()
        .id(("item", id_hash))
        .w_48()
        .flex()
        .flex_col()
        .gap_2()
        .p_2()
        .bg(rgb(0x313244))
        .rounded_md()
        .hover(|s| s.bg(rgb(0x45475a)))
        .cursor_pointer()
        .on_click(move |_event, _window, cx| {
            let client = client_clone.clone();
            let item_id = item_id.clone();
            let username = username_clone.clone();
            cx.update_global::<AppStateGlobal, _>(move |global, cx| {
                global.0.update(cx, |state, cx| {
                    state.current_view =
                        AppView::ItemDetail(ItemDetailView::new(client, item_id, username, cx));
                });
            });
        })
        .child(
            div()
                .h_64()
                .bg(rgb(0x181825))
                .rounded_sm()
                .overflow_hidden()
                .child(image),
        )
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .overflow_hidden()
                .text_ellipsis()
                .child(item.name.clone().unwrap_or_default()),
        )
        .child(
            div().text_xs().text_color(rgb(0xa6adc8)).child(
                item.production_year
                    .map(|y| y.to_string())
                    .unwrap_or_default(),
            ),
        )
}
