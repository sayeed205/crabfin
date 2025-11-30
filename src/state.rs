use crate::config::Config;

#[derive(Clone, Debug, PartialEq)]
pub enum Screen {
    ServerList,
    AddServer,
    Login(String),
}

pub struct AppState {
    pub config: Config,
    pub screen: Screen,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        let screen = if config.servers.is_empty() {
            Screen::AddServer
        } else {
            Screen::ServerList
        };

        Self {
            config,
            screen,
        }
    }
}
