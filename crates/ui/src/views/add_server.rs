use crate::prelude::*;
use crate::views::login::LoginView;
use jellyfin::{client::ClientBuilder, models::PublicServerInfo, system};
use settings::ServerConfig;

enum AddServerState {
    Idle,
    Validating,
    Validated(PublicServerInfo),
    Error(String),
}

pub struct AddServerView {
    focus_handle: FocusHandle,
    url_input: Entity<TextInput>,
    state: AddServerState,
    validated_url: String,
}

impl AddServerView {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        let url_input = TextInput::new(cx, "http://localhost:8096");
        
        cx.new(|_| Self {
            focus_handle,
            url_input,
            state: AddServerState::Idle,
            validated_url: String::new(),
        })
    }

    fn handle_back(_: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.update_global::<AppStateGlobal, _>(|global, cx| {
            global.0.update(cx, |state, cx| {
                 let view = ServerListView::new(cx);
                 state.current_view = AppView::ServerList(view);
            });
        });
    }

    fn validate_server(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_input.read(cx).content.to_string();
        
        if url.is_empty() {
             self.state = AddServerState::Error("URL cannot be empty".to_string());
             cx.notify();
             return;
        }

        self.state = AddServerState::Validating;
        cx.notify();

        let url_clone = url.clone();
        
        cx.spawn(move |view: gpui::WeakEntity<AddServerView>, cx: &mut gpui::AsyncApp| {
            let mut cx = cx.clone();
            let url_inner = url_clone.clone();
            async move {
                let result = crate::runtime::runtime().spawn(async move {
                    // Simulate async work or real network call
                    let client_res = ClientBuilder::new(&url_inner);
                    
                    match client_res {
                    Ok(builder) => {
                        match builder.build() {
                            Ok(client) => {
                                match system::get_public_info(&client).await {
                                    Ok(info) => Ok(info),
                                    Err(e) => {
                                        tracing::error!("Failed to get public info: {}", e);
                                        Err("Could not reach server. Please check URL.".to_string())
                                    },
                                }
                            },
                            Err(e) => {
                                tracing::error!("Failed to build client: {}", e);
                                Err("Invalid URL format.".to_string())
                            },
                        }
                    },
                    Err(e) => {
                        tracing::error!("Client builder error: {}", e);
                        Err(format!("Invalid URL: {}", e))
                    },
                }
            }).await.unwrap_or_else(|e| Err(format!("Task execution failed: {}", e)));

            view.update(&mut cx, |view, cx| {
                match result {
                    Ok(info) => {
                        view.state = AddServerState::Validated(info);
                        view.validated_url = url_clone;
                    },
                    Err(e) => {
                        view.state = AddServerState::Error(e);
                    }
                }
                cx.notify();
            }).ok();
            }
        }).detach();
    }

    fn save_server(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let (name, id, url) = if let AddServerState::Validated(info) = &self.state {
            (info.server_name.clone(), info.id.clone(), self.validated_url.clone())
        } else {
            return;
        };

        let mut saved_server_config = None;

        cx.update_global::<ConfigGlobal, _>(|config, cx| {
            config.model.update(cx, |model, _cx| {
                let uuid = uuid::Uuid::parse_str(&id).unwrap_or_else(|_| uuid::Uuid::new_v4());
                
                let server_config = ServerConfig {
                    id: uuid,
                    name,
                    url,
                    device_id: uuid::Uuid::new_v4(),
                };
                
                model.add_server(server_config.clone());
                model.settings.default_server_id = Some(uuid);
                let _ = model.save();

                saved_server_config = Some(server_config);
            });
        });
        
        if let Some(server) = saved_server_config {
            cx.update_global::<AppStateGlobal, _>(|global, cx| {
                global.0.update(cx, |state, cx| {
                     let view = LoginView::new(server, cx);
                     state.current_view = AppView::Login(view);
                     cx.notify();
                });
            });
        }
    }
}

impl Render for AddServerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state_view = match &self.state {
            AddServerState::Idle => div(),
            AddServerState::Validating => div().child("Connecting...").text_color(rgb(0xfab387)),
            AddServerState::Validated(info) => div()
                .flex()
                .flex_col()
                .gap_2()
                .child(div().child(format!("Found: {}", info.server_name)).text_color(rgb(0xa6e3a1)))
                .child(div().child(format!("Version: {}", info.version)).text_sm().text_color(rgb(0xa6adc8)))
                .child(
                    button()
                        .size(ButtonSize::Regular)
                        .intent(ButtonIntent::Primary)
                        .child("Save Server")
                        .id("save-server-btn")
                        .on_click(cx.listener(Self::save_server)),
                ),
            AddServerState::Error(msg) => div().child(msg.clone()).text_color(rgb(0xf38ba8)),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .child(div().text_xl().font_weight(FontWeight::BOLD).child("Add Server"))
                    .child(
                         button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Secondary)
                            .child("Back")
                            .id("back-btn")
                            .on_click(|ev, win, cx| Self::handle_back(ev, win, cx)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().child("Server URL"))
                            .child(self.url_input.clone()),
                    )
                    .child(
                        button()
                            .size(ButtonSize::Regular)
                            .intent(ButtonIntent::Primary)
                            .child("Connect")
                            .id("connect-btn")
                            .on_click(cx.listener(Self::validate_server)),
                    )
                    .child(state_view),
            )
    }
}

impl Focusable for AddServerView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
