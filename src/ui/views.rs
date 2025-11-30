/**/use gpui::prelude::*;
use gpui::*;

use super::screens::add_server::AddServerView;
use super::screens::auth::AuthView;
use super::screens::logged_in::LoggedInView;
use super::screens::server_selection::ServerSelectionView;
use super::theme::Theme;
use crate::services::ServiceManager;

enum ViewState {
    ServerSelection,
    AddServer,
    Auth,
    LoggedIn,
    MainApp,
}

/// Main application view that manages the overall UI state
pub struct MainView {
    focus_handle: FocusHandle,
    state: ViewState,
    server_selection_view: Entity<ServerSelectionView>,
    add_server_view: Entity<AddServerView>,
    auth_view: Option<Entity<AuthView>>,
    logged_in_view: Option<Entity<LoggedInView>>,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let service_manager = cx.global::<ServiceManager>();
        let settings_service = service_manager.settings_service().expect("Settings service not initialized");
        let config = settings_service.get_config();

        // Determine initial state based on saved configuration
        let initial_state = if config.servers.is_empty() {
            // No servers configured, go to add server
            ViewState::AddServer
        } else if let Some(_active_server_id) = &config.active_server_id {
            // Has active server, check for session
            if settings_service.get_active_session().is_some() {
                // Has session, go to logged-in view
                ViewState::LoggedIn
            } else {
                // No session, go to auth
                ViewState::Auth
            }
        } else {
            // Has servers but none selected, go to server selection
            ViewState::ServerSelection
        };

        let weak_view = cx.entity().downgrade();

        // Setup server selection view
        let weak_view_for_selection = weak_view.clone();
        let server_selection_view = cx.new(|cx| {
            let weak_view_add = weak_view_for_selection.clone();
            let weak_view_select = weak_view_for_selection.clone();
            ServerSelectionView::new(cx)
                .on_add_server(move |_window, cx| {
                    weak_view_add.update(cx, |view, cx| {
                        view.state = ViewState::AddServer;
                        cx.notify();
                    }).ok();
                })
                .on_select_server(move |server_id, _window, cx| {
                    let weak_view = weak_view_select.clone();
                    let server_id_clone = server_id.clone();

                    cx.spawn(async move |_, mut cx| {
                        let result = cx.update(|cx| {
                            let service_manager = cx.global::<ServiceManager>();
                            if let Some(settings_service) = service_manager.settings_service() {
                                cx.background_executor().spawn({
                                    let settings_service = settings_service.clone();
                                    let server_id = server_id_clone.clone();
                                    async move {
                                        settings_service.set_active_server(server_id).await
                                    }
                                }).detach();
                            }
                        });

                        if result.is_ok() {
                            let _ = weak_view.update(cx, |view, cx| {
                                view.create_auth_view(cx);
                                view.state = ViewState::Auth;
                                cx.notify();
                            });
                        }
                    }).detach();
                })
        });

        // Setup add server view
        let weak_view_for_add = weak_view.clone();
        let add_server_view = cx.new(|cx| {
            let weak_view_cancel = weak_view_for_add.clone();
            let weak_view_connect = weak_view_for_add.clone();
            AddServerView::new(cx)
                .on_cancel(move |_window, cx| {
                    weak_view_cancel.update(cx, |view, cx| {
                        let service_manager = cx.global::<ServiceManager>();
                        let settings_service = service_manager.settings_service().expect("Settings service not initialized");
                        let config = settings_service.get_config();

                        // Only go back to server selection if there are servers
                        if !config.servers.is_empty() {
                            view.state = ViewState::ServerSelection;
                        }
                        cx.notify();
                    }).ok();
                })
                .on_connect(move |url, _window, cx| {
                    let main_view_weak = weak_view_connect.clone();
                    let url_clone = url.clone();
                    let add_view_entity = cx.entity().clone();
                    cx.spawn(async move |_, mut cx| {
                        // Create a temporary client for validation
                        let mut client = crate::client::CrabfinClient::new();

                        match client.connect(&url_clone).await {
                            Ok(info) => {
                                // Add server to settings
                                let server_config = crate::models::server::ServerConfig::new(
                                    info.id.clone(),
                                    info.name,
                                    url_clone.clone(),
                                );

                                let server_id = info.id;
                                let _ = main_view_weak.update(cx, |main_view, cx| {
                                    let service_manager = cx.global::<ServiceManager>();
                                    if let Some(settings_service) = service_manager.settings_service() {
                                        let settings_service_clone = settings_service.clone();
                                        let server_config_clone = server_config.clone();
                                        let server_id_clone = server_id.clone();

                                        cx.background_executor().spawn(async move {
                                            // Add server and set as active
                                            if let Err(e) = settings_service_clone.add_server(server_config_clone).await {
                                                eprintln!("Failed to save server: {}", e);
                                                return;
                                            }

                                            if let Err(e) = settings_service_clone.set_active_server(server_id_clone).await {
                                                eprintln!("Failed to set active server: {}", e);
                                            }
                                        }).detach();
                                    }

                                    // Navigate to auth view
                                    main_view.create_auth_view(cx);
                                    main_view.state = ViewState::Auth;
                                    cx.notify();
                                });
                            }
                            Err(e) => {
                                let _ = add_view_entity.update(cx, |add_view, cx| {
                                    add_view.set_error(format!("Connection failed: {}", e), cx);
                                });
                            }
                        }
                    }).detach();
                })
        });

        let mut view = Self {
            focus_handle: cx.focus_handle(),
            state: initial_state,
            server_selection_view,
            add_server_view,
            auth_view: None,
            logged_in_view: None,
        };

        // Create auth view if needed
        if matches!(view.state, ViewState::Auth) {
            view.create_auth_view(cx);
        }

        // Create logged-in view if needed
        if matches!(view.state, ViewState::LoggedIn) {
            view.create_logged_in_view(cx);
        }

        view
    }

    fn create_auth_view(&mut self, cx: &mut Context<Self>) {
        let service_manager = cx.global::<ServiceManager>();
        let settings_service = service_manager.settings_service().expect("Settings service not initialized");
        let active_server = settings_service.get_active_server();

        if let Some(server) = active_server {
            let weak_view = cx.entity().downgrade();

            let auth_view = cx.new(|cx| {
                let weak_view_back = weak_view.clone();
                let weak_view_login = weak_view.clone();

                AuthView::new(server.name.clone(), cx)
                    .on_back(move |_window, cx| {
                        weak_view_back.update(cx, |view, cx| {
                            view.state = ViewState::ServerSelection;
                            cx.notify();
                        }).ok();
                    })
                    .on_login(move |username, password, remember_me, _window, cx| {
                        let weak_view = weak_view_login.clone();
                        let auth_entity = cx.entity().clone();

                        cx.spawn(async move |_, mut cx| {
                            // Get server URL
                            let server_url = cx.update(|cx| {
                                let service_manager = cx.global::<ServiceManager>();
                                let settings_service = service_manager.settings_service().expect("Settings service not initialized");
                                settings_service.get_active_server().map(|s| s.url)
                            }).ok().flatten();

                            if let Some(url) = server_url {
                                let mut client = crate::client::CrabfinClient::new();

                                // First connect to server
                                if let Err(e) = client.connect(&url).await {
                                    let _ = auth_entity.update(cx, |auth_view, cx| {
                                        auth_view.set_error(format!("Connection failed: {}", e), cx);
                                    });
                                    return;
                                }

                                // Then authenticate
                                match client.authenticate(&username, &password).await {
                                    Ok(auth_response) => {
                                        // Create session object
                                        let session_result = cx.update(|cx| {
                                            let service_manager = cx.global::<ServiceManager>();
                                            if let Some(settings_service) = service_manager.settings_service() {
                                                if let Some(active_server) = settings_service.get_active_server() {
                                                    let session = crate::models::UserSession::new(
                                                        active_server.id.clone(),
                                                        active_server.name.clone(),
                                                        auth_response.user.id.clone(),
                                                        auth_response.user.name.clone(),
                                                        auth_response.user.policy.as_ref()
                                                            .map(|p| p.is_administrator)
                                                            .unwrap_or(false),
                                                        auth_response.access_token.clone(),
                                                    );

                                                    // Only save session if remember_me is checked
                                                    if remember_me {
                                                        cx.background_executor().spawn({
                                                            let settings_service = settings_service.clone();
                                                            let server_id = active_server.id.clone();
                                                            let session_clone = session.clone();
                                                            async move {
                                                                settings_service.save_session(server_id, session_clone).await
                                                            }
                                                        }).detach();
                                                    }

                                                    // Return session and server URL
                                                    Some((session, active_server.url))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        });

                                        if let Ok(Some((session, server_url))) = session_result {
                                            let _ = weak_view.update(cx, |view, cx| {
                                                view.create_logged_in_view_with_session(session, server_url, cx);
                                                view.state = ViewState::LoggedIn;
                                                cx.notify();
                                            });
                                        }
                                    }
                                    Err(e) => {
                                        let _ = auth_entity.update(cx, |auth_view, cx| {
                                            auth_view.set_error(format!("Authentication failed: {}", e), cx);
                                        });
                                    }
                                }
                            }
                        }).detach();
                    })
            });

            self.auth_view = Some(auth_view);
        }
    }

    fn create_logged_in_view(&mut self, cx: &mut Context<Self>) {
        let service_manager = cx.global::<ServiceManager>();
        let settings_service = service_manager.settings_service().expect("Settings service not initialized");

        if let Some(session) = settings_service.get_active_session() {
            if let Some(server) = settings_service.get_active_server() {
                let weak_view = cx.entity().downgrade();

                let logged_in_view = cx.new(|cx| {
                    LoggedInView::new(session, server.url, cx)
                        .on_logout(move |_window, cx| {
                            let weak_view = weak_view.clone();

                            cx.spawn(async move |_, mut cx| {
                                let result = cx.update(|cx| {
                                    let service_manager = cx.global::<ServiceManager>();
                                    if let Some(settings_service) = service_manager.settings_service() {
                                        if let Some(active_server) = settings_service.get_active_server() {
                                            cx.background_executor().spawn({
                                                let settings_service = settings_service.clone();
                                                let server_id = active_server.id.clone();
                                                async move {
                                                    settings_service.clear_session(&server_id).await
                                                }
                                            }).detach();
                                        }
                                    }
                                });

                                if result.is_ok() {
                                    let _ = weak_view.update(cx, |view, cx| {
                                        view.create_auth_view(cx);
                                        view.state = ViewState::Auth;
                                        cx.notify();
                                    });
                                }
                            }).detach();
                        })
                });

                self.logged_in_view = Some(logged_in_view);
            }
        }
    }

    fn create_logged_in_view_with_session(&mut self, session: crate::models::UserSession, server_url: String, cx: &mut Context<Self>) {
        let weak_view = cx.entity().downgrade();

        let logged_in_view = cx.new(|cx| {
            LoggedInView::new(session, server_url, cx)
                .on_logout(move |_window, cx| {
                    let weak_view = weak_view.clone();

                    cx.spawn(async move |_, mut cx| {
                        let result = cx.update(|cx| {
                            let service_manager = cx.global::<ServiceManager>();
                            if let Some(settings_service) = service_manager.settings_service() {
                                if let Some(active_server) = settings_service.get_active_server() {
                                    cx.background_executor().spawn({
                                        let settings_service = settings_service.clone();
                                        let server_id = active_server.id.clone();
                                        async move {
                                            settings_service.clear_session(&server_id).await
                                        }
                                    }).detach();
                                }
                            }
                        });

                        if result.is_ok() {
                            let _ = weak_view.update(cx, |view, cx| {
                                view.create_auth_view(cx);
                                view.state = ViewState::Auth;
                                cx.notify();
                            });
                        }
                    }).detach();
                })
        });

        self.logged_in_view = Some(logged_in_view);
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("main-view")
            .key_context("main-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(theme.background())
            .text_color(theme.on_background())
            .child(
                match &self.state {
                    ViewState::ServerSelection => self.server_selection_view.clone().into_any_element(),
                    ViewState::AddServer => self.add_server_view.clone().into_any_element(),
                    ViewState::Auth => {
                        if let Some(auth_view) = &self.auth_view {
                            auth_view.clone().into_any_element()
                        } else {
                            div().child("Loading...").into_any_element()
                        }
                    }
                    ViewState::LoggedIn => {
                        if let Some(logged_in_view) = &self.logged_in_view {
                            logged_in_view.clone().into_any_element()
                        } else {
                            div().child("Loading...").into_any_element()
                        }
                    }
                    ViewState::MainApp => div().child("Main App Content").into_any_element(),
                }
            )
    }
}

impl Focusable for MainView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub fn init(_cx: &mut App) {
    // No actions to register yet
}