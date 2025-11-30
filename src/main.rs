mod config;
mod views;
mod state;

use config::Server;
use gpui::*;
use gpui_component::*;
use state::{AppState, Screen};
use views::{AddServerView, LoginView, ServerListView};

struct CrabfinApp {
    state: Entity<AppState>,
}

impl CrabfinApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_| AppState::new());

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        }).detach();

        Self { state }
    }

    fn render_active_view(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = self.state.read(cx);
        match &state.screen {
            Screen::AddServer => {
                let state_model = self.state.clone();
                self.create_add_server_view(state_model, window, cx).into_any_element()
            }
            Screen::Login(url) => {
                let state_model = self.state.clone();
                self.create_login_view(state_model, url.clone(), window, cx).into_any_element()
            }
            Screen::ServerList => {
                let state_model = self.state.clone();
                self.create_server_list_view(state_model, window, cx).into_any_element()
            }
        }
    }

    fn create_add_server_view(&self, state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Entity<AddServerView> {
        cx.new(|cx| {
            AddServerView::new(
                window,
                cx,
                {
                    let state = state.clone();
                    move |url, _window, cx| {
                        state.update(cx, |state, cx| {
                            state.screen = Screen::Login(url);
                            cx.notify();
                        });
                    }
                },
                {
                    let state = state.clone();
                    move |_window, cx| {
                        state.update(cx, |state, cx| {
                            if !state.config.servers.is_empty() {
                                state.screen = Screen::ServerList;
                                cx.notify();
                            }
                        });
                    }
                },
            )
        })
    }

    fn create_login_view(&self, state: Entity<AppState>, url: String, window: &mut Window, cx: &mut Context<Self>) -> Entity<LoginView> {
        cx.new(|cx| {
            LoginView::new(
                window,
                cx,
                url.clone(),
                {
                    let state = state.clone();
                    move |username, _password, _window, cx| {
                        let server = Server {
                            id: cuid2::create_id(),
                            name: "Jellyfin Server".to_string(),
                            url: url.clone(),
                            access_token: Some("dummy_token".to_string()),
                            user_id: Some(username),
                        };

                        state.update(cx, |state, cx| {
                            state.config.add_server(server);
                            let _ = state.config.save();
                            state.screen = Screen::ServerList;
                            cx.notify();
                        });
                    }
                },
                {
                    let state = state.clone();
                    move |_window, cx| {
                        state.update(cx, |state, cx| {
                            state.screen = Screen::AddServer;
                            cx.notify();
                        });
                    }
                },
            )
        })
    }

    fn create_server_list_view(&self, state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Entity<ServerListView> {
        let servers = state.read(cx).config.servers.clone();
        cx.new(|cx| {
            ServerListView::new(
                servers,
                |server, _window, _cx| {
                    println!("Selected server: {:?}", server);
                },
                {
                    let state = state.clone();
                    move |_window, cx| {
                        state.update(cx, |state, cx| {
                            state.screen = Screen::AddServer;
                            cx.notify();
                        });
                    }
                },
            )
        })
    }
}

impl Render for CrabfinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.render_active_view(window, cx))
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let app = cx.new(|cx| {
                    cx.observe_window_appearance(window, |_, window, cx| {
                        Theme::sync_system_appearance(Some(window), cx);
                    })
                        .detach();

                    Theme::sync_system_appearance(Some(window), cx);

                    CrabfinApp::new(window, cx)
                });
                cx.new(|cx| Root::new(app, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
            .detach();
    });
}
