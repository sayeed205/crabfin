use crate::ui::theme::Theme;
use gpui::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Filled,
    Tonal,
    Outlined,
    Text,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<Icon>, // Placeholder for Icon type if we have one, or just use AnyElement
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    disabled: bool,
    full_width: bool,
}

// Placeholder for Icon
pub struct Icon;

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Filled,
            size: ButtonSize::Medium,
            icon: None,
            on_click: None,
            disabled: false,
            full_width: false,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // Colors based on variant
        let (bg_color, text_color, border_color) = match self.variant {
            ButtonVariant::Filled => (
                if self.disabled { theme.on_surface().opacity(0.12) } else { theme.primary() },
                if self.disabled { theme.on_surface().opacity(0.38) } else { theme.on_primary() },
                None
            ),
            ButtonVariant::Tonal => (
                if self.disabled { theme.on_surface().opacity(0.12) } else { theme.secondary_container() },
                if self.disabled { theme.on_surface().opacity(0.38) } else { theme.on_secondary_container() },
                None
            ),
            ButtonVariant::Outlined => (
                gpui::transparent_black(),
                if self.disabled { theme.on_surface().opacity(0.38) } else { theme.primary() },
                Some(if self.disabled { theme.on_surface().opacity(0.12) } else { theme.outline() })
            ),
            ButtonVariant::Text => (
                gpui::transparent_black(),
                if self.disabled { theme.on_surface().opacity(0.38) } else { theme.primary() },
                None
            ),
        };

        // Sizing
        let (height, px_padding) = match self.size {
            ButtonSize::Small => (px(32.0), px(12.0)),
            ButtonSize::Medium => (px(40.0), px(24.0)),
            ButtonSize::Large => (px(48.0), px(32.0)),
        };

        let mut el = div()
            .id(self.label.clone())
            .h(height)
            .px(px_padding)
            .flex()
            .items_center()
            .justify_center()
            .rounded(height / 2.0) // Pill shape
            .bg(bg_color)
            .cursor_pointer();

        if self.full_width {
            el = el.w_full();
        }

        if let Some(border) = border_color {
            el = el.border_1().border_color(border);
        }

        if !self.disabled {
            el = el
                .hover(|style| style.opacity(0.92)) // State layer opacity
                .active(|style| style.opacity(0.88));

            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        } else {
            el = el.cursor_not_allowed().opacity(0.38);
        }

        let label_div = div()
            .text_color(text_color)
            .font_weight(FontWeight::MEDIUM)
            .line_height(relative(1.0));

        let label_div = match self.size {
            ButtonSize::Small => label_div.text_xs(),
            ButtonSize::Medium => label_div.text_sm(),
            ButtonSize::Large => label_div.text_base(),
        };

        el.child(
            label_div.child(self.label)
        )
    }
}
