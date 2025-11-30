use crate::config::{SavedUser, Server};
use gpui::*;
use gpui_component::{button::ButtonVariants, ActiveTheme, StyledExt};

pub struct UserSelectionView {
    server: Server,
    saved_users: Vec<SavedUser>,
    on_user_selected: Box<dyn Fn(SavedUser, &mut Window, &mut Context<UserSelectionView>) + 'static>,
    on_back: Box<dyn Fn(&mut Window, &mut Context<UserSelectionView>) + 'static>,
}

impl UserSelectionView {
    pub fn new(
        server: Server,
        _window: &mut Window,
        _cx: &mut Context<Self>,
        on_user_selected: impl Fn(SavedUser, &mut Window, &mut Context<UserSelectionView>) + 'static,
        on_back: impl Fn(&mut Window, &mut Context<UserSelectionView>) + 'static,
    ) -> Self {
        let saved_users = server.saved_users.clone();
        Self {
            server,
            saved_users,
            on_user_selected: Box::new(on_user_selected),
            on_back: Box::new(on_back),
        }
    }
}

impl Render for UserSelectionView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.background)
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().text_xl().font_bold().child(format!("Select User - {}", self.server.name)))
                    .child(
                        div()
                            .w_full() // Added width constraint here
                            .flex()
                            .gap_4()
                            .flex_wrap()
                            .justify_center()
                            .children(if self.saved_users.is_empty() {
                                vec![
                                    gpui_component::button::Button::new("login_new_user")
                                        .primary()
                                        .label("Login as New User")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            use crate::config::SavedUser;
                                            let dummy_user = SavedUser {
                                                username: String::new(),
                                                user_id: String::new(),
                                            };
                                            (this.on_user_selected)(dummy_user, window, cx);
                                        }))
                                        .into_any_element()
                                ]
                            } else {
                                vec![]
                            })
                            .children(self.saved_users.iter().enumerate().map(|(idx, user)| {
                                let user_clone = user.clone();
                                div()
                                    .id(("user", idx))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_2()
                                    .p_4()
                                    .rounded_md()
                                    .hover(|s| s.bg(theme.muted))
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .size_24()
                                            .rounded_full()
                                            .bg(theme.secondary)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xl()
                                            .child(user.username.chars().next().unwrap_or('?').to_string())
                                    )
                                    .child(user.username.clone())
                                    .on_click(cx.listener({
                                        let user = user_clone;
                                        move |this, _, window, cx| {
                                            (this.on_user_selected)(user.clone(), window, cx);
                                        }
                                    }))
                            }))
                    )
                    .child(
                        gpui_component::button::Button::new("back")
                            .label("Back")
                            .on_click(cx.listener(|this, _, window, cx| {
                                (this.on_back)(window, cx);
                            }))
                    )
            )
    }
}
