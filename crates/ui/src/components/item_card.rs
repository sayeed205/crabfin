use crate::prelude::*;
use jellyfin::models::BaseItemDto;

pub fn item_card(item: &BaseItemDto) -> impl IntoElement {
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
