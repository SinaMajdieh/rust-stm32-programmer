use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, Icon, Sizable,
        alert::Alert,
        button::{Button, ButtonVariants},
    },
    div,
};

pub struct DeployFail;

impl DeployFail {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for DeployFail {
    fn render(
        &mut self,
        _window: &mut gpui_kit::Window,
        cx: &mut gpui_kit::prelude::Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .size_full()
            .v_flex()
            .justify_start()
            .border_1()
            .border_color(theme.colors.border)
            .rounded_md()
            .px_8()
            .py_12()
            .gap_8()
            // Header
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_start()
                    .gap_4()
                    .child(
                        Icon::new(IconName::CircleX)
                            .size_12()
                            .text_color(theme.colors.danger),
                    )
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(
                                div()
                                    .text_3xl()
                                    .text_color(theme.colors.danger)
                                    .child("Deployment Failed"),
                            )
                            .child(
                                div()
                                    .text_color(theme.colors.muted_foreground)
                                    .text_center()
                                    .text_align(gpui_kit::TextAlign::Center)
                                    .child("The firmware was built successfully, but could not be programmed."),
                            ),
                    ),
            )
            // Error message
            .child(
                Alert::error(
                    "banner-error",
                    "Error: ST-LINK connection failed",
                )
                .text_lg()
                .banner()
                .icon(Icon::empty()),
            ).child(
                v_flex()
                    .items_stretch()
                    .justify_start()
                    .gap_3()
                    .child(div().text_3xl().child("Try these steps:"))
                    .child(
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                h_flex()
                                    .justify_start()
                                    .items_center()
                                    .gap_2()
                                    .text_color(theme.colors.muted_foreground)
                                    .child(Icon::new(IconName::CircleSmall))
                                    .child("Check USB cable and ST-LINK is detected")
                            ).child(
                                h_flex()
                                    .justify_start()
                                    .items_center()
                                    .gap_2()
                                    .text_color(theme.colors.muted_foreground)
                                    .child(Icon::new(IconName::CircleSmall))
                                    .child("Make sure the board is powered")
                            ).child(
                                h_flex()
                                    .justify_start()
                                    .items_center()
                                    .gap_2()
                                    .text_color(theme.colors.muted_foreground)
                                    .child(Icon::new(IconName::CircleSmall))
                                    .child("Try a different USB port")
                            )
                    )
            )
            .child(
                h_flex()
                    .items_stretch()
                    .justify_between()
                    .flex_wrap()
                    .gap_4()
                    .min_h_16()
                    .child(
                        Button::new("back-to-project")
                            .large()
                            .text_lg()
                            .h_full()
                            .flex_1()
                            .min_w_32()
                            .label("Back to Project")
                            .rounded_md(),
                    )
                    .child(
                        Button::new("retry")
                            .large()
                            .text_lg()
                            .h_full()
                            .flex_1()
                            .min_w_32()
                            .primary()
                            .label("Retry Deployment")
                            .icon(Icon::new(IconName::RotateCw).large())
                            .rounded_md(),
                    ),
            )
    }
}
