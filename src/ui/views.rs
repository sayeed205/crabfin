use gpui::prelude::*;
use gpui::*;

use super::components::ui::button::{Button, ButtonVariant};
use super::components::ui::text_input::TextInput;
use super::theme::Theme;


/// Main application view that manages the overall UI state
pub struct MainView {
    focus_handle: FocusHandle,
    input_view: Entity<TextInput>,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input_view = cx.new(|cx| {
            TextInput::new(cx)
                .with_placeholder("Enter your username")
                .on_change(|value, _, _| {
                    println!("Input changed: {}", value);
                })
        });

        Self {
            focus_handle: cx.focus_handle(),
            input_view,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("main-view")
            .key_context("main-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background())
            .text_color(theme.on_background())
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        Button::new("Filled Button")
                            .variant(ButtonVariant::Filled)
                            .on_click(|_, _, _| println!("Filled clicked"))
                    )
                    .child(
                        Button::new("Tonal Button")
                            .variant(ButtonVariant::Tonal)
                            .on_click(|_, _, _| println!("Tonal clicked"))
                    )
                    .child(
                        Button::new("Outlined Button")
                            .variant(ButtonVariant::Outlined)
                            .on_click(|_, _, _| println!("Outlined clicked"))
                    )
                    .child(
                        Button::new("Text Button")
                            .variant(ButtonVariant::Text)
                            .on_click(|_, _, _| println!("Text clicked"))
                    )
            )
            .child(
                div()
                    .w_full()
                    .max_w(px(300.0))
                    .child(self.input_view.clone())
            )
    }
}

impl Focusable for MainView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub fn init(_cx: &mut App) {
    // No actions to register yet
}