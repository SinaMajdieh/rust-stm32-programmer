//! STM32 Studio UI entry point.
//!
//! This executable uses a small demonstration backend so the interface can be
//! developed independently from the real generation/build/programming crate.

use std::rc::Rc;

use slint::ComponentHandle;

mod backend;
mod config;

slint::include_modules!();

use backend::DemoBackend;

#[derive(Debug, Clone, Copy)]
enum PipelineStage {
    Ready = 0,
    Generating = 1,
    Building = 2,
    Programming = 3,
    Complete = 4,
}

impl PipelineStage {
    fn value(self) -> i32 {
        self as i32
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let backend = Rc::new(DemoBackend::new());

    {
        let weak = ui.as_weak();
        let backend = Rc::clone(&backend);
        ui.on_generate_and_flash(move || {
            let Some(ui) = weak.upgrade() else { return };
            backend.generate_and_flash(&ui, ui.get_prompt());
        });
    }

    {
        let weak = ui.as_weak();
        ui.on_new_project(move || {
            let Some(ui) = weak.upgrade() else { return };
            ui.set_has_project(false);
            ui.set_busy(false);
            ui.set_pipeline_stage(PipelineStage::Ready.value());
            ui.set_status_message("Ready".into());
            ui.set_output_open(false);
            ui.set_prompt("".into());
        });
    }

    {
        let weak = ui.as_weak();
        ui.on_toggle_output(move || {
            let Some(ui) = weak.upgrade() else { return };
            ui.set_output_open(!ui.get_output_open());
        });
    }

    // The device-settings callback is intentionally a no-op in the demo.
    ui.on_device_settings(|| {});

    ui.run()?;
    Ok(())
}
