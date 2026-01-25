use crate::prelude::*;
use crate::views::home::HomeView;
use crate::views::login::LoginView;
use jellyfin::client::{AuthenticatedClient, ClientBuilder};
use settings::{ServerConfig, UserConfig};

pub struct UserSelectionView {
    focus_handle: FocusHandle,
    server: ServerConfig,
    users: Vec<UserConfig>,
    error_msg: Option<String>,
}

impl UserSelectionView {
    pub fn new(server: ServerConfig, users: Vec<UserConfig>, cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        cx.new(|_| Self {
            focus_handle,
            server,
            users,
            error_msg: None,
        })
    }

    fn go_to_login(&self, cx: &mut Context<Self>) {
        let server = self.server.clone();
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::Login(LoginView::new(server, cx));
                cx.notify();
            });
        });
    }

    fn handle_back(_: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                state.current_view = AppView::ServerList(ServerListView::new(cx));
                cx.notify();
            });
        });
    }

    fn handle_user_click(&mut self, user_idx: usize, cx: &mut Context<Self>) {
        if user_idx >= self.users.len() {
            return;
        }
        let user = &self.users[user_idx];

        if !user.remember_me {
            self.go_to_login(cx);
            return;
        }

        let token = match credentials::get_token(self.server.id, &user.id) {
            Ok(t) => t,
            Err(_) => {
                self.go_to_login(cx);
                return;
            }
        };

        let server = self.server.clone();
        let user_id = user.id.clone();
        let username = user.username.clone();

        cx.spawn(move |view: gpui::WeakEntity<UserSelectionView>, cx: &mut gpui::AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let is_valid = crate::runtime::runtime().spawn(async move {
                    if let Ok(builder) = ClientBuilder::new(&server.url) {
                        if let Ok(client) = builder.device_id(server.device_id.to_string()).build() {
                            let auth_client = AuthenticatedClient::from_token(client, token, user_id);
                            return jellyfin::system::validate_session(&auth_client).await.unwrap_or(false);
                        }
                    }
                    false
                }).await.unwrap_or(false);

                view.update(&mut cx, |view, cx| {
                    if is_valid {
                        cx.update_global::<ConfigGlobal, _>(|config, cx| {
                            config.model.update(cx, |model, _| {
                                model.settings.last_server_id = Some(view.server.id);
                                model.settings.last_user_id = Some(view.users[user_idx].id.clone());
                                let _ = model.save();
                            });
                        });

                        let server_name = view.server.name.clone();
                        cx.update_global::<AppStateGlobal, _>(|global, cx| {
                            global.0.update(cx, |state, cx| {
                                state.current_view = AppView::Home(HomeView::new(username, server_name, cx));
                                cx.notify();
                            });
                        });
                    } else {
                        view.error_msg = Some("Session expired. Please login again.".to_string());
                        cx.notify();
                    }
                }).ok();
            }
        }).detach();
    }
}

impl Render for UserSelectionView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child(format!("Select User - {}", self.server.name)),
                    )
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .child("Back")
                            .id("back-btn")
                            .on_click(|ev, win, cx| Self::handle_back(ev, win, cx)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .max_w(px(400.0))
                    .children(
                        self.users.iter().enumerate().map(|(idx, user)| {
                            let username = user.username.clone();
                            div()
                                .flex()
                                .items_center()
                                .p_4()
                                .bg(rgb(0x313244))
                                .rounded_md()
                                .cursor_pointer()
                                .child(username)
                                .id(SharedString::from(format!("user-{}", idx)))
                                .on_click(cx.listener(move |view, _, _, cx| view.handle_user_click(idx, cx)))
                        })
                    )
            )
            .child(
                 div()
                    .p_4()
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .child("Login as different user")
                            .id("login-diff-user-btn")
                            .on_click(cx.listener(|view, _, _, cx| view.go_to_login(cx))),
                    )
            )
            .child(
                if let Some(msg) = &self.error_msg {
                    div().p_4().text_color(rgb(0xf38ba8)).child(msg.clone())
                } else {
                    div()
                }
            )
    }
}

impl Focusable for UserSelectionView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
