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
                    // TODO: Validate and add server
                    weak_view_connect.update(cx, |view, cx| {
                        view.state = ViewState::MainApp;
                        cx.notify();
                    }).ok();
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