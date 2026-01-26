use crate::components::image_display::image_display;
use crate::prelude::*;
use jellyfin::client::{AuthenticatedClient, ImageType};
use jellyfin::models::BaseItemDto;

pub fn episode_card(
    item: &BaseItemDto,
    client: AuthenticatedClient,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let index = item.index_number.unwrap_or(0);
    let name = item.name.clone().unwrap_or_default();
    let runtime_ticks = item.run_time_ticks.unwrap_or(0);
    let runtime_mins = runtime_ticks / 10_000 / 1000 / 60;
    let played = item
        .user_data
        .as_ref()
        .and_then(|d| d.played)
        .unwrap_or(false);

    let image = if let Some(tag) = item.image_tags.as_ref().and_then(|t| t.get("Primary")) {
        image_display(
            item.id.clone(),
            ImageType::Primary,
            tag.clone(),
            128, // w_32
            None,
            client,
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
        .flex()
        .items_center()
        .gap_4()
        .p_3()
        .bg(rgb(0x313244))
        .rounded_md()
        .hover(|s| s.bg(rgb(0x45475a)))
        .cursor_pointer()
        .child(
            div()
                .w_8()
                .flex()
                .justify_center()
                .text_color(rgb(0xa6adc8))
                .child(index.to_string()),
        )
        .child(
            div()
                .w_32()
                .h_20() // 16:9 approx
                .bg(rgb(0x181825))
                .rounded_sm()
                .overflow_hidden()
                .child(image),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .child(div().font_weight(FontWeight::SEMIBOLD).child(name))
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa6adc8))
                        .child(format!("{} min", runtime_mins)),
                ),
        )
        .child(if played {
            div()
                .text_xs()
                .px_2()
                .py_1()
                .bg(rgb(0xa6e3a1))
                .text_color(rgb(0x1e1e2e))
                .rounded_sm()
                .child("Watched")
        } else {
            div()
        })
}
