use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, h_flex, v_flex},
    component::{ActiveTheme, Icon, Sizable, progress::Progress},
    div,
    prelude::FluentBuilder,
};

use crate::components::log::Log;

pub struct BuildProgress {
    progress: f32,
    log: Entity<Log>,
}

impl BuildProgress {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let progress = -1.;
        let log = cx.new(|_| Log::new().max_h_48());

        log.update(cx, |log, cx| {
            log.log("10:42:01", "Starting firmware build...", cx);
            log.log("10:42:02", "Checking target: STM32F103C8T6", cx);
            log.log("10:42:03", "arm-none-eabi-gcc -c main.c", cx);
            log.log("10:42:04", "arm-none-eabi-gcc -c stm32f1xx_hal.c", cx);
            log.log("10:42:05", "Compiling startup_stm32f103xb.s", cx);
            log.log("10:42:06", "Linking firmware.elf...", cx);
            log.log("10:42:07", "Generating firmware.bin...", cx);
            log.log("10:42:08", "Build completed successfully.", cx);
        });
        Self { progress, log }
    }
}

impl Render for BuildProgress {
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
                    .child(div().child(Icon::new(IconName::SquareTerminal).size_12()))
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(div().text_3xl().child("Building your Project"))
                            .child(
                                div()
                                    .text_color(theme.colors.muted_foreground)
                                    .text_center()
                                    .text_align(gpui_kit::TextAlign::Center)
                                    .child("Compiling the firmware for STM32"),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .gap_4()
                    .child(
                        Progress::new("build")
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
            .child(self.log.clone())
    }
}
