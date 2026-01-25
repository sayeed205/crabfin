use crate::views::{add_server::AddServerView, server_list::ServerListView};
use gpui::*;
use settings::{Config, SettingsWatcher};

pub struct ConfigGlobal {
    pub model: Entity<Config>,
    pub watcher: Option<Entity<SettingsWatcher>>,
}

impl Global for ConfigGlobal {}

#[derive(Clone)]
pub enum AppView {
    ServerList(Entity<ServerListView>),
    AddServer(Entity<AddServerView>),
}

pub struct AppState {
    pub current_view: AppView,
}

pub struct AppStateGlobal(pub Entity<AppState>);

impl Global for AppStateGlobal {}
