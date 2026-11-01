use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, Colorize, Icon, Sizable,
        button::{Button, ButtonVariants},
        stepper::{Stepper, StepperItem},
    },
    div,
};

pub struct DeploySuccess;

impl DeploySuccess {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for DeploySuccess {
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
                v_flex()
                    .w_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div().child(
                            Icon::new(IconName::CircleCheckBig)
                                .size_12()
                                .text_color(theme.colors.success),
                        ),
                    )
                    .child(div().text_3xl().child("Deployment successful!"))
                    .child(
                        div()
                            .text_color(theme.colors.muted_foreground)
                            .text_center()
                            .text_align(gpui_kit::TextAlign::Center)
                            .child("Your firmware has been programmed and verified."),
                    ),
            )
            .child(
                h_flex()
                    .items_stretch()
                    .justify_between()
                    .flex_wrap()
                    .gap_4()
                    .min_h_16()
                    .child(
                        Button::new("view-files")
                            .large()
                            .text_lg()
                            .h_full()
                            .flex_1()
                            .min_w_32()
                            .label("View Files")
                            .rounded_md(),
                    )
                    .child(
                        Button::new("back-to-project")
                            .large()
                            .text_lg()
                            .h_full()
                            .flex_1()
                            .min_w_32()
                            .primary()
                            .label("Back to Project")
                            .rounded_md(),
                    ),
            )
            .child(
                div()
                    .p_8()
                    .bg(theme.colors.secondary.darken(0.25))
                    .border_color(theme.colors.border)
                    .border_1()
                    .rounded_md()
                    .child(
                        Stepper::new("stepper")
                            .large()
                            .disabled(true)
                            .selected_index(3)
                            .text_center(true)
                            .items([
                                StepperItem::new()
                                    .icon(Icon::new(IconName::Check).large())
                                    .child(v_flex().items_center().child("Generate").child(
                                        div().text_color(theme.colors.muted_foreground).child(""),
                                    )),
                                StepperItem::new()
                                    .icon(Icon::new(IconName::Check).large())
                                    .child(v_flex().items_center().child("Build").child(
                                        div().text_color(theme.colors.muted_foreground).child(""),
                                    )),
                                StepperItem::new()
                                    .icon(Icon::new(IconName::Check).large())
                                    .child(v_flex().items_center().child("Deploy").child(
                                        div().text_color(theme.colors.muted_foreground).child(""),
                                    )),
                                StepperItem::new()
                                    .icon(Icon::new(IconName::Sparkle).large())
                                    .child(v_flex().items_center().child("Done").child(
                                        div().text_color(theme.colors.muted_foreground).child(""),
                                    )),
                            ]),
                    ),
            )
    }
}
