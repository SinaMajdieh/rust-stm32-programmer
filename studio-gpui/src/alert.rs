use gpui_kit::{
    App, AppContext, Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, Window, component::WindowExt, div, prelude::FluentBuilder,
};

pub struct AlertContent {
    message: String,
    details: Option<String>,
    details_open: bool,
}

impl AlertContent {
    pub fn new(message: String, details: Option<String>) -> Self {
        Self {
            message,
            details,
            details_open: false,
        }
    }
}

impl Render for AlertContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(div().text_sm().child(self.message.clone()))
            .when_some(self.details.clone(), |this, details| {
                let details_open = self.details_open;

                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .cursor_pointer()
                                .text_xs()
                                .child(if details_open {
                                    "Details ▾"
                                } else {
                                    "Details ▸"
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.details_open = !this.details_open;
                                        cx.notify();
                                    }),
                                ),
                        )
                        .when(details_open, |this| {
                            this.child(
                                div()
                                    .p_3()
                                    .rounded_md()
                                    .text_xs()
                                    .font_family(".SystemUIMonospace")
                                    .child(details),
                            )
                        }),
                )
            })
    }
}

pub fn show_alert(
    window: &mut Window,
    cx: &mut App,
    title: impl Into<String>,
    message: impl Into<String>,
    details: Option<String>,
) {
    let title = title.into();
    let message = message.into();

    let content = cx.new(|_| AlertContent::new(message, details));

    window.open_alert_dialog(cx, move |alert, _, _| {
        let title = title.clone();
        let content = content.clone();

        alert
            .title(title)
            .content(move |dialog, _, _| dialog.child(content.clone()))
            .on_ok(|_, _, _| true)
    });
}
