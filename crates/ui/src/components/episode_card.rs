use crate::prelude::*;
use jellyfin::models::BaseItemDto;

pub fn episode_card(item: &BaseItemDto) -> impl IntoElement {
    let index = item.index_number.unwrap_or(0);
    let name = item.name.clone().unwrap_or_default();
    let runtime_ticks = item.run_time_ticks.unwrap_or(0);
    let runtime_mins = runtime_ticks / 10_000 / 1000 / 60;
    let played = item
        .user_data
        .as_ref()
        .and_then(|d| d.played)
        .unwrap_or(false);

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
