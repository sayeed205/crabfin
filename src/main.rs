mod config;
mod views;
mod components;
mod state;
mod api;
mod credentials;

use config::Server;
use gpui::*;
use gpui_component::*;
use state::AppState;
use views::{AddServerView, AppLayout, LoginView, ServerListView, UserSelectionView};

struct CrabfinApp {
    state: Entity<AppState>,
    active_view: AnyView,
}

impl CrabfinApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_| AppState::new());
        let weak_app = cx.weak_entity();

        let config = state.read(cx).config.clone();
        let active_server = config.active_server_id.as_ref()
            .and_then(|id| config.servers.iter().find(|s| &s.id == id).cloned());

        let active_view = if let Some(server) = active_server {
            // We have an active server, start with UserSelectionView
            // Check for last connected user to auto-login
            if let Some(user_id) = config.last_connected_user_id.clone() {
                if let Some(user) = server.saved_users.iter().find(|u| u.user_id == user_id).cloned() {
                     let weak_app = weak_app.clone();
                     let server_url = server.url.clone();
                     let user_id = user.user_id.clone();
                     let username = user.username.clone();
                     let server_clone = server.clone();

                     cx.spawn_in(&*window, move |_, cx: &mut AsyncWindowContext| {
                        let weak_app = weak_app.clone();
                        let server_url = server_url.clone();
                        let user_id = user_id.clone();
                        let mut cx = cx.clone();
                        async move {
                            if let Ok(_token) = credentials::read_token(&server_url, &user_id).await {
                                let _ = cx.update_window_entity(&weak_app.upgrade().unwrap(), |app, window, cx| {
                                    app.active_view = Self::create_app_layout_view(weak_app.clone(), server_clone, window, cx);
                                    cx.notify();
                                });
                            } else {
                                // Token invalid or missing, go to login
                                let _ = cx.update_window_entity(&weak_app.upgrade().unwrap(), |app, window, cx| {
                                    app.active_view = Self::create_login_view(weak_app.clone(), server_url, Some(username), window, cx);
                                    cx.notify();
                                });
                            }
                        }
                     }).detach();
                }
            }
            
            Self::create_user_selection_view(weak_app.clone(), server, window, cx)
        } else if config.servers.is_empty() {
            Self::create_add_server_view(weak_app.clone(), window, cx)
        } else {
            Self::create_server_list_view(weak_app.clone(), config.servers, window, cx)
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
                                                    saved_users: Vec::new(),
                                                };

                                                app.state.update(cx, |state, _cx| {
                                                    state.config.add_server(server);
                                                    let _ = state.config.save();
                                                });

                                                // Navigate to login
                                                app.active_view = Self::create_login_view(weak_app.clone(), url, None, window, cx);
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

    fn create_login_view(weak_app: WeakEntity<Self>, url: String, username: Option<String>, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|cx| {
            LoginView::new(
                window,
                cx,
                url.clone(),
                username,
                {
                    let weak_app = weak_app.clone();
                    let url = url.clone();
                    move |username, password, remember_me, window, cx| {
                        let weak_app = weak_app.clone();
                        let url = url.clone();
                        let view = cx.weak_entity();

                        cx.spawn_in(&*window, move |_, mut cx: &mut AsyncWindowContext| {
                            let mut cx = cx.clone();
                            async move {
                                let _ = view.update(&mut cx, |view, cx| {
                                    view.set_loading(true, cx);
                                    view.set_error(None, cx);
                                });

                                match api::authenticate(&url, &username, &password).await {
                                    Ok(auth_response) => {
                                        if let Some(app_entity) = weak_app.upgrade() {
                                            cx.update_window_entity(&app_entity, |app, window, cx| {
                                                let user_id = auth_response.user.id.clone();
                                                let access_token = auth_response.access_token.clone();

                                                let (server_id, server_clone) = {
                                                    let server = app.state.read(cx).config.servers.iter().find(|s| s.url == url).cloned();
                                                    if let Some(s) = server {
                                                        (Some(s.id.clone()), Some(s))
                                                    } else {
                                                        (None, None)
                                                    }
                                                };

                                                if let Some(server_id) = server_id {
                                                    if remember_me {
                                                        app.state.update(cx, |state, _cx| {
                                                            // Add saved user to config
                                                            state.config.add_saved_user(&server_id, username.clone(), user_id.clone());
                                                            let _ = state.config.save();
                                                        });

                                                        // Save token to keyring (run async in background)
                                                        let url_for_keyring = url.clone();
                                                        let user_id_for_keyring = user_id.clone();
                                                        let token_for_keyring = access_token.clone();
                                                        cx.spawn_in(&*window, |_, _cx: &mut AsyncWindowContext| async move {
                                                            let _ = credentials::write_token(&url_for_keyring, &user_id_for_keyring, &token_for_keyring).await;
                                                        }).detach();
                                                    }

                                                    if let Some(server) = server_clone {
                                                        app.active_view = Self::create_app_layout_view(weak_app.clone(), server, window, cx);
                                                    }
                                                } else {
                                                    // Server not found, return to server list
                                                    let servers = app.state.read(cx).config.servers.clone();
                                                    app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                                                }
                                                cx.notify();
                                            }).ok();
                                        }
                                    }
                                    Err(e) => {
                                        let _ = view.update(&mut cx, |view, cx| {
                                            view.set_loading(false, cx);
                                            view.set_error(Some(format!("Login failed: {}", e)), cx);
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
                            // If we have servers, go back to list, else add server
                            let servers = app.state.read(cx).config.servers.clone();
                            if !servers.is_empty() {
                                app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                            } else {
                                app.active_view = Self::create_add_server_view(weak_app.clone(), window, cx);
                            }
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
                        let _ = weak_app.update(cx, |app, cx| {
                            // Save active server
                            app.state.update(cx, |state, _cx| {
                                state.config.set_active_server(server.id.clone());
                                let _ = state.config.save();
                            });
                            
                            app.active_view = Self::create_user_selection_view(weak_app.clone(), server.clone(), window, cx);
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

    fn create_app_layout_view(weak_app: WeakEntity<Self>, server: Server, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|cx| {
            AppLayout::new(
                server.clone(),
                window,
                cx,
                {
                    let weak_app = weak_app.clone();
                    let server = server.clone();
                    move |window, cx| {
                        let server = server.clone();
                        let _ = weak_app.update(cx, |app, cx| {
                            // Clear last connected user
                            app.state.update(cx, |state, _cx| {
                                state.config.clear_last_connected_user();
                                let _ = state.config.save();
                            });

                            app.active_view = Self::create_user_selection_view(weak_app.clone(), server, window, cx);
                            cx.notify();
                        });
                    }
                }
            )
        }).into()
    }

    fn create_user_selection_view(weak_app: WeakEntity<Self>, server: Server, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        cx.new(|cx| {
            UserSelectionView::new(
                server.clone(),
                window,
                cx,
                {
                    let weak_app = weak_app.clone();
                    let server = server.clone();
                    move |user, window, cx| {
                        let server = server.clone();
                        let weak_app = weak_app.clone();
                        let _ = weak_app.update(cx, |app, cx| {
                            // Check for saved token in keyring
                            let server_url = server.url.clone();
                            let user_id = user.user_id.clone();
                            let username = user.username.clone();

                            let weak_app_for_spawn = weak_app.clone();
                            let server_url_for_async = server_url.clone();
                            let user_id_for_async = user_id.clone();

                            cx.spawn_in(&*window, move |_, cx: &mut AsyncWindowContext| {
                                let weak_app = weak_app_for_spawn.clone();
                                let server_url = server_url_for_async.clone();
                                let user_id = user_id_for_async.clone();
                                let mut cx = cx.clone();
                                async move {
                                    if let Ok(_token) = credentials::read_token(&server_url, &user_id).await {
                                        // Token found, try to auto-login
                                        let _ = cx.update_window_entity(&weak_app.upgrade().unwrap(), |app, window, cx| {
                                            // Save last connected user
                                            app.state.update(cx, |state, _cx| {
                                                state.config.set_last_connected_user(user_id.clone());
                                                let _ = state.config.save();
                                            });

                                            if let Some(server) = app.state.read(cx).config.servers.iter().find(|s| s.url == server_url).cloned() {
                                                app.active_view = Self::create_app_layout_view(weak_app, server, window, cx);
                                                cx.notify();
                                            }
                                        });
                                    } else {
                                        // No token found or error reading it, navigate to login
                                        let _ = cx.update_window_entity(&weak_app.upgrade().unwrap(), |app, window, cx| {
                                            app.active_view = Self::create_login_view(weak_app, server_url, Some(username), window, cx);
                                            cx.notify();
                                        });
                                    }
                                }
                            }).detach();
                        });
                    }
                },
                {
                    let weak_app = weak_app.clone();
                    move |window, cx| {
                        let _ = weak_app.update(cx, |app, cx| {
                            // Clear active server when going back
                            app.state.update(cx, |state, _cx| {
                                state.config.clear_active_server();
                                let _ = state.config.save();
                            });

                            // If we have servers, go back to list, else add server
                            let servers = app.state.read(cx).config.servers.clone();
                            if !servers.is_empty() {
                                app.active_view = Self::create_server_list_view(weak_app.clone(), servers, window, cx);
                            } else {
                                app.active_view = Self::create_add_server_view(weak_app.clone(), window, cx);
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
