use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

mod app;

// struct CrabfinApp removed

fn main() {
    Application::new().run(|cx: &mut App| {
        app::setup_config(cx);
        ui::components::text_input::bind_actions(cx);

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
            |_, cx| cx.new(|_| app::CrabfinApp),
        )
        .unwrap();
        cx.activate(true);
    });
}
