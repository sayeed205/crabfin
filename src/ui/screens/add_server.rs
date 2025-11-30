use gpui::prelude::*;
use gpui::*;

use crate::ui::components::ui::button::{Button, ButtonVariant};
use crate::ui::components::ui::text_input::TextInput;
use crate::ui::theme::Theme;

pub struct AddServerView {
    focus_handle: FocusHandle,
    input: Entity<TextInput>,
    error_message: Option<String>,
    is_connecting: bool,
    on_connect: Option<Box<dyn Fn(String, &mut Window, &mut Context<Self>) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + 'static>>,
}

impl AddServerView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            TextInput::new(cx)
                .with_placeholder("https://jellyfin.example.com")
        });

        Self {
            focus_handle: cx.focus_handle(),
            input,
            error_message: None,
            is_connecting: false,
            on_connect: None,
            on_cancel: None,
        }
    }

    pub fn set_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.error_message = Some(message);
        self.is_connecting = false;
        cx.notify();
    }

    pub fn set_connecting(&mut self, connecting: bool, cx: &mut Context<Self>) {
        self.is_connecting = connecting;
        cx.notify();
    }

    pub fn on_connect(mut self, handler: impl Fn(String, &mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_connect = Some(Box::new(handler));
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl Render for AddServerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("add-server-view")
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
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .font_family(theme.font_family())
                    .child("Add Server")
            )
            .child(
                div()
                    .w_full()
                    .max_w(px(400.0))
                    .child(self.input.clone())
            )
            .children(
                self.error_message.as_ref().map(|msg| {
                    div()
                        .text_color(theme.error())
                        .child(msg.clone())
                })
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        Button::new("Cancel")
                            .variant(ButtonVariant::Text)
                            .disabled(self.is_connecting)
                            .on_click(cx.listener(|this, _, window, cx| {
                                if let Some(callback) = &this.on_cancel {
                                    callback(window, cx);
                                }
                            }))
                    )
                    .child(
                        Button::new(if self.is_connecting { "Connecting..." } else { "Connect" })
                            .variant(ButtonVariant::Filled)
                            .disabled(self.is_connecting)
                            .on_click(cx.listener(|this, _, window, cx| {
                                if this.is_connecting {
                                    return;
                                }
                                let url = this.input.read(cx).content.to_string();
                                if let Some(callback) = &this.on_connect {
                                    this.is_connecting = true;
                                    this.error_message = None;
                                    cx.notify();
                                    callback(url, window, cx);
                                }
                            }))
                    )
            )
    }
}

impl Focusable for AddServerView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
