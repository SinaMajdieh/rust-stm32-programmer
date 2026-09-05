use std::thread;

use backend::{Report, validate_firmware};

use crate::{MainWindow, ui};

pub fn start(window: slint::Weak<MainWindow>, file_path: String) {
    thread::spawn(move || {
        {
            let mut report = |event| {
                show_report(&window, event);
            };

            let _ = validate_firmware(&file_path, &mut report);
        };

        finish(&window);
    });
}

fn show_report(window: &slint::Weak<MainWindow>, report: Report) {
    let _ = window
        .clone()
        .upgrade_in_event_loop(move |window| match report {
            Report::Progress(progress) => {
                window.set_progress_value(progress as f32);
            }
            Report::Log(message) => {
                ui::append_log(&window, message);
            }
        });
}

fn finish(window: &slint::Weak<MainWindow>) {
    let _ = window.clone().upgrade_in_event_loop(move |window| {
        window.set_is_programming(false);
    });
}
