use gpui::prelude::*;
use gpui::*;

use crate::ui::theme::Theme;

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let checked = self.checked;
        let on_change = self.on_change;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .cursor_pointer()
            .on_click(move |_event, window, cx| {
                if let Some(callback) = &on_change {
                    callback(&!checked, window, cx);
                }
            })
            .child(
                // Checkbox indicator
                div()
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(4.0))
                    .border_2()
                    .when(checked, |this| {
                        this.bg(theme.primary())
                            .border_color(theme.primary())
                    })
                    .when(!checked, |this| {
                        this.bg(gpui::transparent_black())
                            .border_color(theme.outline())
                    })
                    .child(
                        // Checkmark
                        div()
                            .when(checked, |this| {
                                this.text_color(theme.on_primary())
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .child("✓")
                            })
                    )
            )
            .child(
                // Label
                div()
                    .text_color(theme.on_surface())
                    .font_family(theme.font_family())
                    .text_sm()
                    .child(self.label)
            )
    }
}
