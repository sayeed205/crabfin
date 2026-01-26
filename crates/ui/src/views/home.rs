use crate::prelude::*;
use crate::views::server_list::ServerListView;

pub struct HomeView {
    focus_handle: FocusHandle,
    username: String,
    server_name: String,
}

impl HomeView {
    pub fn new(username: String, server_name: String, cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self {
            focus_handle,
            username,
            server_name,
        })
    }

    fn handle_logout(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let (server_id, user_id) = cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, _| {
                let s_id = model.settings.last_server_id;
                let u_id = model.settings.last_user_id.clone();

                model.settings.last_server_id = None;
                model.settings.last_user_id = None;
                let _ = model.save();

                (s_id, u_id)
            })
        });

        if let (Some(s_id), Some(u_id)) = (server_id, user_id) {
            credentials::delete_token(s_id, &u_id, cx).detach_and_log_err(cx);
        }

        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::ServerList(ServerListView::new(cx));
                cx.notify();
            });
        });
    }
}

impl Render for HomeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Welcome, {}!", self.username)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xa6adc8))
                    .child(format!("Connected to {}", self.server_name)),
            )
            .child(
                button()
                    .size(ButtonSize::Regular)
                    .intent(ButtonIntent::Secondary)
                    .child("Logout")
                    .id("logout-btn")
                    .on_click(cx.listener(Self::handle_logout)),
            )
    }
}

impl Focusable for HomeView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
