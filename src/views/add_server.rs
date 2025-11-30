use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, input::{Input, InputState}, *};

pub struct AddServerView {
    input: Entity<InputState>,
    on_connect: Box<dyn Fn(String, &mut Window, &mut Context<AddServerView>) + 'static>,
    on_cancel: Box<dyn Fn(&mut Window, &mut Context<AddServerView>) + 'static>,
    is_validating: bool,
    error_message: Option<String>,
}

impl AddServerView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        on_connect: impl Fn(String, &mut Window, &mut Context<AddServerView>) + 'static,
        on_cancel: impl Fn(&mut Window, &mut Context<AddServerView>) + 'static,
    ) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Server URL"));

        Self {
            input,
            on_connect: Box::new(on_connect),
            on_cancel: Box::new(on_cancel),
            is_validating: false,
            error_message: None,
        }
    }

    pub fn set_validating(&mut self, validating: bool, cx: &mut Context<Self>) {
        self.is_validating = validating;
        cx.notify();
    }

    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error_message = error;
        cx.notify();
    }
}

impl Render for AddServerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.background)
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().text_xl().font_bold().child("Add Server"))
                    .child(Input::new(&self.input))
                    .children(self.error_message.as_ref().map(|msg| {
                        div().text_sm().text_color(theme.danger).child(msg.clone())
                    }))
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_cancel)(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("connect")
                                    .primary()
                                    .label(if self.is_validating { "Connecting..." } else { "Connect" })
                                    .disabled(self.is_validating)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        let url = this.input.read(cx).value();
                                        if !url.is_empty() {
                                            (this.on_connect)(url.to_string(), window, cx);
                                        }
                                    }))
                            )
                    )
            )
    }
}
