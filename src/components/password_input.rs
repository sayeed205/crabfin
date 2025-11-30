use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, input::{Input, InputState}, *};

pub struct PasswordInput {
    state: Entity<InputState>,
    is_masked: bool,
    on_toggle: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PasswordInput {
    pub fn new(state: &Entity<InputState>, is_masked: bool) -> Self {
        Self {
            state: state.clone(),
            is_masked,
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PasswordInput {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let is_masked = self.is_masked;
        let on_toggle = self.on_toggle;

        div()
            .w_full()
            .flex()
            .items_center()
            .relative()
            .child(Input::new(&self.state))
            .child(
                div()
                    .absolute()
                    .right_2()
                    .child(
                        Button::new("toggle_password")
                            .icon(if is_masked { IconName::Eye } else { IconName::EyeOff })
                            .ghost()
                            .on_click(move |event, window, cx| {
                                if let Some(handler) = &on_toggle {
                                    (handler)(event, window, cx);
                                }
                            })
                    )
            )
    }
}
