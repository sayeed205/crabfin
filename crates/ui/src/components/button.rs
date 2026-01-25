use gpui::*;

#[derive(Clone, Copy)]
pub enum ButtonSize {
    Regular,
    Large,
}

#[derive(Clone, Copy)]
pub enum ButtonIntent {
    Primary,
    Secondary,
    Danger,
}

#[derive(Clone, Copy)]
pub enum ButtonStyle {
    Regular,
    Minimal,
}

impl ButtonStyle {
    fn base<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        let div = dest.cursor_pointer().flex();

        match self {
            ButtonStyle::Regular => div.shadow_md().rounded(px(4.0)),
            ButtonStyle::Minimal => div.opacity(0.0).rounded(px(4.0)),
        }
    }

    fn hover<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonStyle::Regular => dest,
            ButtonStyle::Minimal => dest.opacity(0.5),
        }
    }

    fn active<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonStyle::Regular => dest,
            ButtonStyle::Minimal => dest.opacity(0.5),
        }
    }
}

impl ButtonSize {
    fn base<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonSize::Regular => dest.px(px(10.0)).py(px(3.0)).text_sm().gap(px(8.0)),
            ButtonSize::Large => dest
                .px(px(12.0))
                .pt(px(4.0))
                .pb(px(3.0))
                .text_sm()
                .gap(px(8.0)),
        }
    }

    fn active<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        dest
    }
}

impl ButtonIntent {
    fn base<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonIntent::Primary => dest.bg(rgb(0x0078D4)).text_color(white()),
            ButtonIntent::Secondary => dest.bg(rgb(0xE1E1E1)).text_color(black()),
            ButtonIntent::Danger => dest.bg(rgb(0xD13438)).text_color(white()),
        }
    }

    fn hover<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonIntent::Primary => dest.bg(rgb(0x106EBE)),
            ButtonIntent::Secondary => dest.bg(rgb(0xE6E6E6)),
            ButtonIntent::Danger => dest.bg(rgb(0xA4262C)),
        }
    }

    fn active<T>(&self, dest: T) -> T
    where
        T: Styled,
    {
        match self {
            ButtonIntent::Primary => dest.bg(rgb(0x005A9E)),
            ButtonIntent::Secondary => dest.bg(rgb(0x979797)),
            ButtonIntent::Danger => dest.bg(rgb(0xA4262C)),
        }
    }
}

#[derive(IntoElement)]
pub struct Button {
    pub(self) div: Div,
    pub(self) style: ButtonStyle,
    pub(self) size: ButtonSize,
    pub(self) intent: ButtonIntent,
}

impl Button {
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn intent(mut self, intent: ButtonIntent) -> Self {
        self.intent = intent;
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn id(self, id: impl Into<ElementId>) -> InteractiveButton {
        InteractiveButton {
            div: self.div.id(id),
            size: self.size,
            style: self.style,
            intent: self.intent,
        }
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let style = self.style;
        let size = self.size;
        let intent = self.intent;

        style.base(size.base(intent.base(self.div.hover(move |v| style.hover(intent.hover(v))))))
    }
}

#[derive(IntoElement)]
pub struct InteractiveButton {
    pub(self) div: Stateful<Div>,
    pub(self) style: ButtonStyle,
    pub(self) size: ButtonSize,
    pub(self) intent: ButtonIntent,
}

impl InteractiveButton {
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn intent(mut self, intent: ButtonIntent) -> Self {
        self.intent = intent;
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.div = self.div.on_click(fun);
        self
    }
}

impl Styled for InteractiveButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl ParentElement for InteractiveButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl RenderOnce for InteractiveButton {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let style = self.style;
        let size = self.size;
        let intent = self.intent;

        style.base(
            size.base(
                intent.base(
                    self.div
                        .hover(move |v| style.hover(intent.hover(v)))
                        .active(move |v| style.active(size.active(intent.active(v)))),
                ),
            ),
        )
    }
}

pub fn button() -> Button {
    Button {
        div: div(),
        style: ButtonStyle::Regular,
        size: ButtonSize::Regular,
        intent: ButtonIntent::Secondary,
    }
}
