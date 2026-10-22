use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, v_flex},
    component::{
        ActiveTheme, Colorize, Sizable,
        stepper::{Stepper, StepperItem},
    },
    div, rems, rgb,
};

const ICON_SIZE_REM: f32 = 3.75;

pub struct Progress {
    // icon: IconName,
    // title: String,
    // description: String,
}

impl Progress {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for Progress {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div().p_12().child(
            div()
                .v_flex()
                .rounded_md()
                .border_1()
                .border_color(theme.colors.border)
                .bg(theme.colors.background.darken(0.2))
                .p_4()
                .gap_4()
                .child(
                    div()
                        .h_flex()
                        .items_center()
                        .justify_start()
                        .gap_4()
                        .px_4()
                        .py_4()
                        .child(
                            div()
                                .h_full()
                                .text_size(rems(ICON_SIZE_REM))
                                .text_color(cx.theme().colors.primary_active)
                                .child(IconName::Sparkles),
                        )
                        .child(
                            div()
                                .v_flex()
                                .child(div().text_lg().child("Generating Code ..."))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(cx.theme().colors.muted_foreground)
                                        .child("The model is creating your project files"),
                                ),
                        ),
                )
                .child(
                    div().pl(rems(ICON_SIZE_REM / 2.0)).child(
                        Stepper::new("vertical-stepper")
                            .vertical()
                            .selected_index(2)
                            .items_start()
                            .items([
                                StepperItem::new()
                                    .icon(IconName::Check)
                                    .pb_6()
                                    .child(div().pl_4().child("Step 1")),
                                StepperItem::new()
                                    .icon(IconName::Check)
                                    .pb_6()
                                    .child(div().pl_4().child("Step 2")),
                                StepperItem::new()
                                    .pb_6()
                                    .child(div().pl_4().child("Step 3")),
                                StepperItem::new()
                                    .pb_6()
                                    .child(div().pl_4().child("Step 4")),
                            ]),
                    ),
                ),
        )
    }
}
