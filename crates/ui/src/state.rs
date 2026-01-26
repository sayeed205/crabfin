use crate::image_store::ImageStore;
use crate::views::{
    add_server::AddServerView, home::HomeView, item_detail::ItemDetailView,
    item_grid::ItemGridView, library::LibraryView, login::LoginView, season::SeasonView,
    server_list::ServerListView, user_selection::UserSelectionView,
};
use gpui::*;
use settings::{Config, SettingsWatcher};

pub struct ConfigGlobal {
    pub model: Entity<Config>,
    pub watcher: Option<Entity<SettingsWatcher>>,
}

impl Global for ConfigGlobal {}

pub struct ImageStoreGlobal(pub Entity<ImageStore>);
impl Global for ImageStoreGlobal {}

#[derive(Clone)]
pub enum AppView {
    ServerList(Entity<ServerListView>),
    AddServer(Entity<AddServerView>),
    Login(Entity<LoginView>),
    Home(Entity<HomeView>),
    UserSelection(Entity<UserSelectionView>),
    Library(Entity<LibraryView>),
    ItemGrid(Entity<ItemGridView>),
    ItemDetail(Entity<ItemDetailView>),
    Season(Entity<SeasonView>),
}

pub struct AppState {
    pub current_view: AppView,
}

pub struct AppStateGlobal(pub Entity<AppState>);

impl Global for AppStateGlobal {}
