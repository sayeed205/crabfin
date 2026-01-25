use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

struct CrabfinApp {
    title: SharedString,
}

impl Render for CrabfinApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xcdd6f4))
            .child(format!("{}", &self.title))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(768.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Crabfin".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| CrabfinApp {
                    title: "Crabfin".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
