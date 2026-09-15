use gpui_kit::{base::StyledExt, component::ActiveTheme, prelude::*, *};

pub enum HomeEvent {
    NewProject,
    OpenProject,
}

pub struct Home;

impl EventEmitter<HomeEvent> for Home {}

impl Home {
    pub fn new() -> Self {
        Self
    }
}

impl Render for Home {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div().flex().justify_center().w_full().pt_6().child(
            div()
                .flex()
                .gap_4()
                // New Project
                .child(
                    div()
                        .id("new-project")
                        .v_flex()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .w_64()
                        .h_48()
                        .p_4()
                        .rounded(theme.radius)
                        .bg(theme.background)
                        .border_1()
                        .border_color(theme.border)
                        .cursor_pointer()
                        .hover(|this| this.bg(theme.muted))
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(HomeEvent::NewProject);
                        }))
                        .child(div().text_2xl().text_color(theme.foreground).child("+"))
                        .child(
                            div()
                                .font_semibold()
                                .text_xl()
                                .text_color(theme.foreground)
                                .child("New Project"),
                        )
                        .child(
                            div()
                                .text_lg()
                                .text_color(theme.muted_foreground)
                                .child("Create a new project"),
                        ),
                )
                // Open Project
                .child(
                    div()
                        .id("open-project")
                        .v_flex()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .w_64()
                        .h_48()
                        .p_4()
                        .rounded(theme.radius)
                        .bg(theme.background)
                        .border_1()
                        .border_color(theme.border)
                        .cursor_pointer()
                        .hover(|this| this.bg(theme.muted))
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(HomeEvent::OpenProject);
                        }))
                        .child(div().text_2xl().text_color(theme.foreground).child("↗"))
                        .child(
                            div()
                                .font_semibold()
                                .text_xl()
                                .text_color(theme.foreground)
                                .child("Open Project"),
                        )
                        .child(
                            div()
                                .text_lg()
                                .text_color(theme.muted_foreground)
                                .child("Open an existing project"),
                        ),
                ),
        )
    }
}
