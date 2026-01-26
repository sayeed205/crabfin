use gpui::*;
use settings::Config;
use ui::state::{AppStateGlobal, AppView, ConfigGlobal};
use credentials;
use jellyfin::client::{AuthenticatedClient, ClientBuilder};
use jellyfin::system;
use ui::views::{
    add_server::AddServerView, home::HomeView, login::LoginView, server_list::ServerListView,
    user_selection::UserSelectionView,
};

pub fn setup_config(cx: &mut App) {
    let config = Config::load().unwrap_or_default();
    let model = cx.new(|_| config.clone());

    cx.set_global(ConfigGlobal {
        model,
        watcher: None,
    });

    let initial_view = if config.servers.is_empty() {
        AppView::AddServer(AddServerView::new(cx))
    } else if config.settings.default_server_id.is_none()
        && config.settings.last_server_id.is_none()
        && config.servers.len() > 1
    {
        AppView::ServerList(ServerListView::new_selection_mode(cx))
    } else {
        let server_id = config
            .settings
            .last_server_id
            .or(config.settings.default_server_id)
            .or(config.servers.first().map(|s| s.id))
            .expect("Servers not empty but no ID found");

        if let Some(server) = config.get_server(server_id) {
            let users = config.get_users_for_server(server_id);
            let users_clone = users.into_iter().cloned().collect::<Vec<_>>();

            if users_clone.is_empty() {
                AppView::Login(LoginView::new(server.clone(), cx))
            } else {
                AppView::UserSelection(UserSelectionView::new(server.clone(), users_clone, cx))
            }
        } else {
            AppView::ServerList(ServerListView::new(cx))
        }
    };

    let app_state = cx.new(|_cx| ui::state::AppState {
        current_view: initial_view,
    });
    cx.set_global(AppStateGlobal(app_state));

    if let (Some(server_id), Some(user_id)) = (
        config.settings.last_server_id,
        config.settings.last_user_id.clone(),
    ) {
        let server_config = config.get_server(server_id).cloned();
        let user_config = config.get_user(&user_id, server_id).cloned();

        if let (Some(server), Some(user)) = (server_config, user_config) {
            if user.remember_me {
                let token_task = credentials::get_token(server_id, &user_id, cx);
                let server_url = server.url.clone();
                let device_id = server.device_id.to_string();
                let user_id_inner = user_id.clone();
                let username = user.username.clone();

                cx.spawn(move |cx: &mut AsyncApp| {
                    let cx = cx.clone();
                    async move {
                        let token = match token_task.await {
                            Ok(t) => t,
                            Err(e) => {
                                tracing::warn!("Failed to retrieve stored token: {}", e);
                                return;
                            }
                        };

                        let valid_client = ui::runtime::runtime()
                            .spawn(async move {
                                let client_builder = ClientBuilder::new(&server_url);
                                if let Ok(builder) = client_builder {
                                    let client = builder.device_id(device_id).build();
                                    if let Ok(client) = client {
                                        let auth_client = AuthenticatedClient::from_token(
                                            client,
                                            token,
                                            user_id_inner,
                                        );
                                        if system::validate_session(&auth_client)
                                            .await
                                            .unwrap_or(false)
                                        {
                                            return Some(auth_client);
                                        }
                                    }
                                }
                                None
                            })
                            .await;

                        if let Ok(Some(auth_client)) = valid_client {
                            let _ = cx.update_global::<AppStateGlobal, _>(|global, cx| {
                                global.0.update(cx, |state, cx| {
                                    state.current_view =
                                        AppView::Home(HomeView::new(username, auth_client, cx));
                                    cx.notify();
                                });
                            });
                        } else {
                            let _ = cx.update_global::<ConfigGlobal, _>(|config, cx| {
                                config.model.update(cx, |model, _| {
                                    model.settings.last_server_id = None;
                                    model.settings.last_user_id = None;
                                    let _ = model.save();
                                });
                            });
                        }
                    }
                })
                .detach();
            }
        }
    }
}

pub struct CrabfinApp;

impl Render for CrabfinApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state_global = cx.global::<AppStateGlobal>();
        let view = app_state_global.0.read(cx).current_view.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .child(match view {
                AppView::ServerList(v) => v.into_any_element(),
                AppView::AddServer(v) => v.into_any_element(),
                AppView::Login(v) => v.into_any_element(),
                AppView::Home(v) => v.into_any_element(),
                AppView::UserSelection(v) => v.into_any_element(),
                AppView::Library(v) => v.into_any_element(),
                AppView::ItemGrid(v) => v.into_any_element(),
                AppView::ItemDetail(v) => v.into_any_element(),
                AppView::Season(v) => v.into_any_element(),
            })
    }
}
