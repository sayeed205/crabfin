use crate::prelude::*;

pub struct ItemGridView {
    focus_handle: FocusHandle,
}

impl ItemGridView {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self { focus_handle })
    }
}

impl Render for ItemGridView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child("Item Grid View (Not Implemented)")
    }
}

impl Focusable for ItemGridView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
