use crate::components::AppSidebar;
use crate::config::Server;
use gpui::*;

pub struct AppLayout {
    sidebar: Entity<AppSidebar>,
}

impl AppLayout {
    pub fn new(server: Server, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_cx| AppSidebar::new(server));
        Self { sidebar }
    }
}

impl Render for AppLayout {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(self.sidebar.clone())
            .child(
                div()
                    .flex_1()
                    .bg(gpui::white()) // Temporary background to distinguish
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("Main Content Area")
            )
    }
}
