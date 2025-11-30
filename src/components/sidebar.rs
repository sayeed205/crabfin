use crate::config::Server;
use gpui::*;
use gpui_component::{sidebar::*, IconName, Side};

pub struct AppSidebar {
    current_server: Server,
}

impl AppSidebar {
    pub fn new(current_server: Server) -> Self {
        Self {
            current_server,
        }
    }
}

impl Render for AppSidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child("User Profile")
            )
    }
}
