mod config;
mod views;
mod state;
mod api;

use config::Server;
use gpui::*;
use gpui_component::*;
use state::AppState;
use views::{AddServerView, LoginView, ServerListView};

struct CrabfinApp {
    state: Entity<AppState>,
    active_view: AnyView,
}

impl CrabfinApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_| AppState::new());
        let weak_app = cx.weak_entity();

        let active_view = if state.read(cx).config.servers.is_empty() {
            Self::create_add_server_view(weak_app.clone(), window, cx)
        } else {
            let servers = state.read(cx).config.servers.clone();
            Self::create_server_list_view(weak_app.clone(), servers, window, cx)
        };

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        }).detach();

        Self { state, active_view }
    }

    fn render_active_view(&self, _window: &mut Window, _cx: &mut Context<Self>) -> AnyElement {
        self.active_view.clone().into_any_element()
    }

    fn create_add_server_view(weak_app: WeakEntity<Self>, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|cx| {
            AddServerView::new(
                window,
                cx,
                {
                    let weak_app = weak_app.clone();
                    move |url, window, cx| {
                        let weak_app = weak_app.clone();
                        let url = url.clone();
                        let view = cx.weak_entity();

                        // Use spawn_in to get AsyncWindowContext which allows access to window and app
                        cx.spawn_in(&*window, |_, mut cx: &mut AsyncWindowContext| {
                            let mut cx = cx.clone();
                            async move {
                                // Set validating state
                                let _ = view.update(&mut cx, |view, cx| {
                                    view.set_validating(true, cx);
                                    view.set_error(None, cx);
                                });

                                match api::validate_server(&url).await {
                                    Ok(info) => {
                                        if let Some(app_entity) = weak_app.upgrade() {
                                            cx.update_window_entity(&app_entity, |app, window, cx| {
                                                // Check if server already exists
                                                if app.state.read(cx).config.servers.iter().any(|s| s.url == url) {
                                                    let _ = view.update(cx, |view, cx| {
                                                        view.set_validating(false, cx);
                                                        view.set_error(Some("Server already exists".to_string()), cx);
                                                    });
                                                    return;
                                                }

                                                let server = Server {
                                                    id: info.id,
                                                    name: info.server_name,
                                                    url: url.clone(),
                                                    access_token: None,
                                                    user_id: None,
                                                };

                                                app.state.update(cx, |state, cx| {
                                                    state.config.add_server(server);
                                                    let _ = state.config.save();
                                                });

                                                // Navigate to login
                                                app.active_view = Self::create_login_view(weak_app.clone(), url, window, cx);
                                                cx.notify();
                                            }).ok();
                                        }
                                    }
                                    Err(e) => {
                                        let _ = view.update(&mut cx, |view, cx| {
                                            view.set_validating(false, cx);
                                            view.set_error(Some(format!("Failed to connect: {}", e)), cx);
                                        });
                                    }
                                }
                            }
                        }).detach();
                    }
                },
                {
                    let weak_app = weak_app.clone();
                    move |window, cx| {
                        let _ = weak_app.update(cx, |app, cx| {
                            let servers = app.state.read(cx).config.servers.clone();
                            if !servers.is_empty() {
                                app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                                cx.notify();
                            }
                        });
                    }
                },
            )
        })
            .into()
    }

    fn create_login_view(weak_app: WeakEntity<Self>, url: String, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|cx| {
            LoginView::new(
                window,
                cx,
                url.clone(),
                {
                    let weak_app = weak_app.clone();
                    let url = url.clone();
                    move |username, _password, window, cx| {
                        let _ = weak_app.update(cx, |app, cx| {
                            // TODO: Implement actual login
                            // For now just update the server with user_id and token
                            app.state.update(cx, |state, _cx| {
                                if let Some(server) = state.config.servers.iter_mut().find(|s| s.url == url) {
                                    server.user_id = Some(username);
                                    server.access_token = Some("dummy_token".to_string());
                                    let _ = state.config.save();
                                }
                            });

                            let servers = app.state.read(cx).config.servers.clone();
                            app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                            cx.notify();
                        });
                    }
                },
                {
                    let weak_app = weak_app.clone();
                    move |window, cx| {
                        let _ = weak_app.update(cx, |app, cx| {
                            app.active_view = Self::create_add_server_view(weak_app.clone(), window, cx);
                            cx.notify();
                        });
                    }
                },
            )
        })
            .into()
    }

    fn create_server_list_view(weak_app: WeakEntity<Self>, servers: Vec<Server>, _window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|_cx| {
            ServerListView::new(
                servers,
                {
                    let weak_app = weak_app.clone();
                    move |server, window, cx| {
                        let server_url = server.url.clone();
                        let access_token = server.access_token.clone();

                        let _ = weak_app.update(cx, |app, cx| {
                            if access_token.is_some() {
                                // Already logged in, navigate to main content
                                // For now, just go to a dummy login view (or dashboard in future)
                                // Re-using login view for now as placeholder for "Connected" state
                                app.active_view = Self::create_login_view(weak_app.clone(), server_url, window, cx);
                            } else {
                                // Not logged in, navigate to login view
                                app.active_view = Self::create_login_view(weak_app.clone(), server_url, window, cx);
                            }
                            cx.notify();
                        });
                    }
                },
                {
                    let weak_app = weak_app.clone();
                    move |window, cx| {
                        let _ = weak_app.update(cx, |app, cx| {
                            app.active_view = Self::create_add_server_view(weak_app.clone(), window, cx);
                            cx.notify();
                        });
                    }
                },
                {
                    let weak_app = weak_app.clone();
                    move |server, window, cx| {
                        let server_url = server.url.clone();
                        let _ = weak_app.update(cx, |app, cx| {
                            app.state.update(cx, |state, _cx| {
                                if let Some(index) = state.config.servers.iter().position(|s| s.url == server_url) {
                                    state.config.servers.remove(index);
                                    let _ = state.config.save();
                                }
                            });

                            // Refresh the view
                            let servers = app.state.read(cx).config.servers.clone();
                            if servers.is_empty() {
                                app.active_view = Self::create_add_server_view(weak_app.clone(), window, cx);
                            } else {
                                app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                            }
                            cx.notify();
                        });
                    }
                },
            )
        })
            .into()
    }
}

impl Render for CrabfinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.render_active_view(window, cx))
    }
}

fn main() {
    // Initialize Tokio runtime for reqwest
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = runtime.enter();

    let app = Application::new().with_assets(gpui_component_assets::Assets);

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
