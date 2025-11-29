use gpui::prelude::*;
use gpui::*;

use super::features::add_server::AddServerView;
use super::features::server_selection::ServerSelectionView;
use super::theme::Theme;
use crate::services::ServiceManager;

enum ViewState {
    ServerSelection,
    AddServer,
    MainApp,
}

/// Main application view that manages the overall UI state
pub struct MainView {
    focus_handle: FocusHandle,
    state: ViewState,
    server_selection_view: Entity<ServerSelectionView>,
    add_server_view: Entity<AddServerView>,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let service_manager = cx.global::<ServiceManager>();
        let settings_service = service_manager.settings_service().expect("Settings service not initialized");
        let config = settings_service.get_config();

        let initial_state = if config.servers.is_empty() {
            ViewState::AddServer
        } else {
            ViewState::ServerSelection
        };

        let weak_view = cx.entity().downgrade();

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
                    println!("Selected server: {}", server_id);
                    // TODO: Connect to server
                    weak_view_select.update(cx, |view, cx| {
                        view.state = ViewState::MainApp;
                        cx.notify();
                    }).ok();
                })
        });

        let weak_view_for_add = weak_view.clone();
        let add_server_view = cx.new(|cx| {
            let weak_view_cancel = weak_view_for_add.clone();
            let weak_view_connect = weak_view_for_add.clone();
            AddServerView::new(cx)
                .on_cancel(move |_window, cx| {
                    weak_view_cancel.update(cx, |view, cx| {
                        view.state = ViewState::ServerSelection;
                        cx.notify();
                    }).ok();
                })
                .on_connect(move |url, _window, cx| {
                    println!("Connecting to: {}", url);

                    let main_view_weak = weak_view_connect.clone();
                    let url_clone = url.clone();
                    let add_view_entity = cx.entity().clone();
                    cx.spawn(async move |_, mut cx| {
                        // Create a temporary client for validation
                        let mut client = crate::client::CrabfinClient::new();

                        match client.connect(&url_clone).await {
                            Ok(info) => {
                                println!("Successfully connected to: {}", info.name);

                                // Add server to settings
                                let server_config = crate::models::server::ServerConfig::new(
                                    info.id,
                                    info.name,
                                    url_clone.clone(),
                                );

                                let _ = main_view_weak.update(cx, |main_view, cx| {
                                    let service_manager = cx.global::<ServiceManager>();
                                    if let Some(settings_service) = service_manager.settings_service() {
                                        let _ = cx.background_executor().spawn({
                                            let settings_service = settings_service.clone();
                                            let server_config = server_config.clone();
                                            async move {
                                                if let Err(e) = settings_service.add_server(server_config).await {
                                                    eprintln!("Failed to save server: {}", e);
                                                }
                                            }
                                        }).detach();
                                    }

                                    main_view.state = ViewState::MainApp;
                                    cx.notify();
                                });
                            }
                            Err(e) => {
                                println!("Failed to connect: {}", e);
                                let _ = add_view_entity.update(cx, |add_view, cx| {
                                    add_view.set_error(format!("Connection failed: {}", e), cx);
                                });
                            }
                        }
                    }).detach();
                })
        });

        Self {
            focus_handle: cx.focus_handle(),
            state: initial_state,
            server_selection_view,
            add_server_view,
        }
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
                match self.state {
                    ViewState::ServerSelection => self.server_selection_view.clone().into_any_element(),
                    ViewState::AddServer => self.add_server_view.clone().into_any_element(),
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