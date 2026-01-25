use crate::config::Server;
use gpui::*;
use gpui_component::{sidebar::*, Icon, IconName, Side};

pub struct AppSidebar {
    current_server: Server,
    on_logout: Box<dyn Fn(&mut Window, &mut Context<AppSidebar>) + 'static>,
}

impl AppSidebar {
    pub fn new(
        current_server: Server,
        on_logout: impl Fn(&mut Window, &mut Context<AppSidebar>) + 'static,
    ) -> Self {
        Self {
            current_server,
            on_logout: Box::new(on_logout),
        }
    }
}

impl Render for AppSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new(Side::Left)
            .header(
                SidebarHeader::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(self.current_server.name.clone())
                    )
            )
            .child(
                SidebarGroup::new("Library")
                    .child(
                        SidebarMenu::new()
                            .child(SidebarMenuItem::new("Home").icon(IconName::LayoutDashboard))
                            .child(SidebarMenuItem::new("Movies").icon(IconName::File))
                            .child(SidebarMenuItem::new("TV Shows").icon(IconName::Frame))
                    )
            )
            .footer(
                SidebarFooter::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child("User Profile")
                            .child(
                                div()
                                    .id("logout_button")
                                    .child(Icon::new(IconName::LogOut).size_4())
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        (this.on_logout)(window, cx);
                                    }))
                            )
                    )
            )
    }
}
