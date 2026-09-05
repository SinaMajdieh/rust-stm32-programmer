//! Small UI demo backend.
//!
//! Replace this type with an adapter around the real project services. The UI
//! should consume high-level state rather than invoking OpenOCD, the compiler,
//! or the model client directly.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use slint::{ComponentHandle, SharedString, Timer, TimerMode};

use crate::{AppWindow, PipelineStage};

pub(crate) struct DemoBackend {
    timer: Timer,
}

impl DemoBackend {
    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::default(),
        }
    }

    pub(crate) fn generate_and_flash(&self, ui: &AppWindow, prompt: SharedString) {
        if prompt.trim().is_empty() || ui.get_busy() {
            return;
        }

        let weak = ui.as_weak();
        let timer = Timer::default();
        let progress = Rc::new(Cell::new(0_i32));
        let progress_for_timer = Rc::clone(&progress);
        let mut stage = PipelineStage::Generating;

        ui.set_busy(true);
        ui.set_has_project(false);
        ui.set_pipeline_stage(stage.value());
        ui.set_status_message("Generating firmware...".into());

        self.timer
            .start(TimerMode::Repeated, Duration::from_millis(60), move || {
                let Some(ui) = weak.upgrade() else {
                    timer.stop();
                    return;
                };

                let next = progress_for_timer.get() + 5;
                progress_for_timer.set(next);

                if next < 100 {
                    return;
                }

                progress_for_timer.set(0);

                match stage {
                    PipelineStage::Generating => {
                        stage = PipelineStage::Building;
                        ui.set_pipeline_stage(stage.value());
                        ui.set_status_message("Building project...".into());
                    }
                    PipelineStage::Building => {
                        stage = PipelineStage::Programming;
                        ui.set_pipeline_stage(stage.value());
                        ui.set_status_message("Programming STM32F103C8T6...".into());
                    }
                    PipelineStage::Programming => {
                        ui.set_busy(false);
                        ui.set_has_project(true);
                        ui.set_pipeline_stage(PipelineStage::Complete.value());
                        ui.set_status_message("Programming completed successfully.".into());
                        ui.set_output_open(false);
                        timer.stop();
                    }
                    PipelineStage::Ready | PipelineStage::Complete => timer.stop(),
                }
            });
    }
}
