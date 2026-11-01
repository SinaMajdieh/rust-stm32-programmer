use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, h_flex, v_flex},
    component::{ActiveTheme, Icon, Sizable, progress::Progress},
    div,
    prelude::FluentBuilder,
};

use crate::components::status_marker::{self, StageStatus};

pub struct DeployProgress {
    progress: f32,
}

impl DeployProgress {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let progress = -1.;
        Self { progress }
    }
}

impl Render for DeployProgress {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(div().child(Icon::new(IconName::Microchip).size_12()))
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(div().text_3xl().child("Deploying to your board"))
                            .child(
                                div()
                                    .text_color(theme.colors.muted_foreground)
                                    .text_center()
                                    .text_align(gpui_kit::TextAlign::Center)
                                    .child("Programming the firmware via ST-LINK"),
                            ),
                    ),
            )
            // Tasks
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .gap_4()
                    .child(
                        Progress::new("deploy")
                            .w_full()
                            .value(self.progress)
                            .large()
                            .when(self.progress < 0., |bar| bar.loading(true)),
                    )
                    .child(
                        div()
                            .child(format!("{}%", self.progress))
                            .when(self.progress < 0., |label| label.hidden()),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .p_4()
                    .gap_4()
                    .bg(theme.colors.secondary)
                    .border_color(theme.colors.border)
                    .border_1()
                    .rounded_md()
                    .child(status_marker::stage_marker(
                        "Connect to programmer",
                        StageStatus::Succeeded,
                        cx,
                    ))
                    .child(status_marker::stage_marker(
                        "Erase Flash",
                        StageStatus::Succeeded,
                        cx,
                    ))
                    .child(status_marker::stage_marker(
                        "Program Firmware",
                        StageStatus::Running,
                        cx,
                    ))
                    .child(status_marker::stage_marker(
                        "Verify",
                        StageStatus::Waiting,
                        cx,
                    )),
            )
    }
}
