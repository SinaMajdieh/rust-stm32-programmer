use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, v_flex},
    component::{ActiveTheme, Icon},
    div,
};

use crate::components::status_marker::{self, StageStatus};

pub struct GenerationProgress;

impl GenerationProgress {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for GenerationProgress {
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
                v_flex()
                    .w_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(div().child(Icon::new(IconName::Sparkles).size_12()))
                    .child(div().text_3xl().child("Generating Your Project"))
                    .child(
                        div()
                            .text_color(theme.colors.muted_foreground)
                            .text_center()
                            .text_align(gpui_kit::TextAlign::Center)
                            .child(
                                "The AI is creating the source code for your STM32 project.\nThis may take a minute.",
                            ),
                    ),
            )
            // Tasks
            .child(
                div().v_flex().p_4().gap_4().bg(theme.colors.secondary).border_color(theme.colors.border).border_1().rounded_md().child(
                    status_marker::stage_marker("Analyzing your prompt", StageStatus::Succeeded, cx)
                ).child(
                    status_marker::stage_marker("Generating code", StageStatus::Running, cx)
                )
                .child(
                    status_marker::stage_marker("Finilizing", StageStatus::Waiting, cx)
                )
            )
    }
}
