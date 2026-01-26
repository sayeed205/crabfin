use crate::prelude::*;
use crate::state::{AppStateGlobal, AppView};
use crate::views::item_detail::ItemDetailView;
use gpui::InteractiveElement;
use jellyfin::client::AuthenticatedClient;
use jellyfin::models::BaseItemDto;
use std::any::Any;

pub fn item_card(
    item: &BaseItemDto,
    client: &AuthenticatedClient,
    username: String,
    cx: &mut Context<impl Any>,
) -> impl IntoElement {
    let item_id = item.id.clone();
    let client = client.clone();
    let username = username.clone();
    let name = item.name.clone().unwrap_or_default();
    let production_year = item
        .production_year
        .map(|y| y.to_string())
        .unwrap_or_default();

    div()
        .w_48()
        .flex()
        .flex_col()
        .gap_2()
        .p_2()
        .bg(rgb(0x313244))
        .rounded_md()
        .hover(|s| s.bg(rgb(0x45475a)))
        .cursor_pointer()
        .on_click(cx.listener(move |_, _, _, cx| {
            let client = client.clone();
            let item_id = item_id.clone();
            let username = username.clone();
            cx.update_global::<AppStateGlobal, _>(move |global, cx| {
                global.0.update(cx, |state, cx| {
                    state.current_view =
                        AppView::ItemDetail(ItemDetailView::new(client, item_id, username, cx));
                    cx.notify();
                });
            });
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
                .child("No Image"),
        )
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .overflow_hidden()
                .text_ellipsis()
                .child(name),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0xa6adc8))
                .child(production_year),
        )
}
