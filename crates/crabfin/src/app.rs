use gpui::*;
use settings::Config;
use ui::state::{AppStateGlobal, AppView, ConfigGlobal};

pub fn setup_config(cx: &mut App) {
    let config = Config::load().unwrap_or_default();
    let model = cx.new(|_| config);

    cx.set_global(ConfigGlobal {
        model,
        watcher: None,
    });

    let app_state = cx.new(|cx| ui::state::AppState {
        current_view: AppView::ServerList(ui::views::server_list::ServerListView::new(cx)),
    });
    cx.set_global(AppStateGlobal(app_state));
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
            })
    }
}
