use gpui::prelude::*;
use gpui::*;

use crate::services::ServiceManager;
use crate::ui::components::ui::button::{Button, ButtonVariant};
use crate::ui::theme::Theme;

pub struct ServerSelectionView {
    focus_handle: FocusHandle,
    on_add_server: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + 'static>>,
    on_select_server: Option<Box<dyn Fn(String, &mut Window, &mut Context<Self>) + 'static>>,
}

impl ServerSelectionView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            on_add_server: None,
            on_select_server: None,
        }
    }

    pub fn on_add_server(mut self, handler: impl Fn(&mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_add_server = Some(Box::new(handler));
        self
    }

    pub fn on_select_server(mut self, handler: impl Fn(String, &mut Window, &mut Context<Self>) + 'static) -> Self {
        self.on_select_server = Some(Box::new(handler));
        self
    }
}

impl Render for ServerSelectionView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let service_manager = cx.global::<ServiceManager>();
        let settings_service = service_manager.settings_service().expect("Settings service not initialized");
        let config = settings_service.get_config();

        div()
            .id("server-selection-view")
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
                    .child("Select Server")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .w_full()
                    .max_w(px(400.0))
                    .children(
                        config.servers.iter().map(|server| {
                            let server_id = server.id.clone();
                            Button::new(server.name.clone())
                                .variant(ButtonVariant::Tonal)
                                .full_width()
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    if let Some(callback) = &this.on_select_server {
                                        callback(server_id.clone(), window, cx);
                                    }
                                }))
                        })
                    )
            )
            .child(
                Button::new("Add Server")
                    .variant(ButtonVariant::Filled)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(callback) = &this.on_add_server {
                            callback(window, cx);
                        }
                    }))
            )
    }
}

impl Focusable for ServerSelectionView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
