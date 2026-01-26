use crate::prelude::*;
use crate::views::library::LibraryView;
use jellyfin::client::ClientBuilder;
use jellyfin::error::JellyfinError;
use settings::{ServerConfig, UserConfig};

enum LoginState {
    Idle,
    Authenticating,
    Success(String),
    Error(String),
}

pub struct LoginView {
    focus_handle: FocusHandle,
    server: ServerConfig,
    username_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    remember_me: bool,
    state: LoginState,
}

impl LoginView {
    pub fn new(server: ServerConfig, cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        let username_input = TextInput::new(cx, "Username");
        let password_input = TextInput::new_password(cx, "Password");

        cx.new(|_| Self {
            focus_handle,
            server,
            username_input,
            password_input,
            remember_me: true,
            state: LoginState::Idle,
        })
    }

    fn handle_back(_: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                let view = ServerListView::new(cx);
                state.current_view = AppView::ServerList(view);
            });
        });
    }

    fn toggle_remember_me(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.remember_me = !self.remember_me;
        cx.notify();
    }

    fn handle_login(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let username = self.username_input.read(cx).content.to_string();
        let password = self.password_input.read(cx).content.to_string();

        if username.is_empty() {
            self.state = LoginState::Error("Username is required".to_string());
            cx.notify();
            return;
        }

        self.state = LoginState::Authenticating;
        cx.notify();

        let server = self.server.clone();
        let remember_me = self.remember_me;

        cx.spawn(move |view: gpui::WeakEntity<LoginView>, cx: &mut gpui::AsyncApp| {
            let mut cx = cx.clone();
            async move {
                let result = crate::runtime::runtime()
                    .spawn(async move {
                        let client = ClientBuilder::new(&server.url)
                            .map_err(|e| format!("Invalid server URL: {}", e))?
                            .device_id(server.device_id.to_string())
                            .build()
                            .map_err(|e| format!("Failed to create client: {}", e))?;

                        match jellyfin::user::authenticate_by_name(&client, &username, &password)
                            .await
                        {
                            Ok(auth_result) => {
                                let auth_client =
                                    jellyfin::client::AuthenticatedClient::new(client, auth_result.clone());
                                Ok((auth_result, server, username, remember_me, auth_client))
                            }
                            Err(JellyfinError::Unauthorized) => {
                                Err("Invalid username or password".to_string())
                            }
                            Err(JellyfinError::Network(_)) => {
                                Err("Could not connect to server".to_string())
                            }
                            Err(e) => Err(format!("Authentication failed: {}", e)),
                        }
                    })
                    .await
                    .unwrap_or_else(|e| Err(format!("Task failed: {}", e)));

                view.update(&mut cx, |view, cx| {
                    match result {
                        Ok((auth_result, server, username, remember_me, auth_client)) => {
                            tracing::info!(
                                "Successfully authenticated {} on {}",
                                auth_result.user.name,
                                server.name
                            );

                            if remember_me {
                                credentials::store_token(
                                    server.id,
                                    &auth_result.user.id,
                                    &auth_result.access_token,
                                    cx,
                                )
                                .detach_and_log_err(cx);
                            }

                            cx.update_global::<ConfigGlobal, _>(|config, cx| {
                                config.model.update(cx, |model, _cx| {
                                    let user_config = UserConfig {
                                        id: auth_result.user.id.clone(),
                                        server_id: server.id,
                                        username,
                                        remember_me,
                                    };
                                    model.add_user(user_config);
                                    model.settings.last_server_id = Some(server.id);
                                    model.settings.last_user_id = Some(auth_result.user.id.clone());
                                    let _ = model.save();
                                });
                            });

                            view.state = LoginState::Success(format!(
                                "Welcome, {}!",
                                auth_result.user.name
                            ));

                            cx.update_global::<AppStateGlobal, _>(|global, cx| {
                                global.0.update(cx, |state, cx| {
                                    state.current_view =
                                        AppView::Library(LibraryView::new(auth_client, cx));
                                    cx.notify();
                                });
                            });
                        }
                        Err(e) => {
                            tracing::error!("Authentication failed: {}", e);
                            view.state = LoginState::Error(e);
                        }
                    }
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state_view = match &self.state {
            LoginState::Idle => div(),
            LoginState::Authenticating => div()
                .child("Authenticating...")
                .text_color(rgb(0xfab387)),
            LoginState::Success(msg) => div().child(msg.clone()).text_color(rgb(0xa6e3a1)),
            LoginState::Error(msg) => div().child(msg.clone()).text_color(rgb(0xf38ba8)),
        };

        let remember_me_color = if self.remember_me {
            rgb(0xa6e3a1)
        } else {
            rgb(0x6c7086)
        };

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
                            .child(format!("Login to {}", self.server.name)),
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
                    .gap_4()
                    .max_w(px(400.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().child("Username"))
                            .child(self.username_input.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().child("Password"))
                            .child(self.password_input.clone()),
                    )
                    .child(
                        div()
                            .id("remember-me-toggle")
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .on_click(cx.listener(Self::toggle_remember_me))
                            .child(
                                div()
                                    .size_4()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(remember_me_color)
                                    .bg(if self.remember_me {
                                        remember_me_color
                                    } else {
                                        rgb(0x1e1e2e)
                                    }),
                            )
                            .child(div().child("Remember me")),
                    )
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Primary)
                            .child("Login")
                            .id("login-btn")
                            .on_click(cx.listener(Self::handle_login)),
                    )
                    .child(state_view),
            )
    }
}

impl Focusable for LoginView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
