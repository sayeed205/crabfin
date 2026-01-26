use crate::views::{
    add_server::AddServerView, home::HomeView, item_grid::ItemGridView, library::LibraryView,
    login::LoginView, server_list::ServerListView, user_selection::UserSelectionView,
};
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
    Login(Entity<LoginView>),
    Home(Entity<HomeView>),
    UserSelection(Entity<UserSelectionView>),
    Library(Entity<LibraryView>),
    ItemGrid(Entity<ItemGridView>),
}

pub struct AppState {
    pub current_view: AppView,
}

pub struct AppStateGlobal(pub Entity<AppState>);

impl Global for AppStateGlobal {}
