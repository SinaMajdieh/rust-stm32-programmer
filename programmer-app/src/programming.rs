use std::{fmt::Display, thread};

use programmer_core::{program_file, ProgrammingProgress};

use crate::{ui, MainWindow};

pub fn start(window: slint::Weak<MainWindow>, file_path: String) {
    thread::spawn(move || {
        let result = program_file(file_path, {
            let window = window.clone();

            move |progress| show_progress(&window, progress)
        });

        show_result(&window, result);
    });
}

fn show_progress(
    window: &slint::Weak<MainWindow>,
    progress: ProgrammingProgress,
) {
    let _ = window.clone().upgrade_in_event_loop(move |window| {
        window.set_progress_value(progress.percentage as f32);
        ui::append_log(&window, progress.text_chunk);
    });
}

fn show_result<E>(
    window: &slint::Weak<MainWindow>,
    result: Result<(), E>,
) where
    E: Display + Send + 'static,
{
    let _ = window.clone().upgrade_in_event_loop(move |window| {
        window.set_is_programming(false);

        let message = match result {
            Ok(()) => "Programming completed successfully.".into(),
            Err(error) => format!("Programming failed: {error}"),
        };

        ui::append_log(&window, message);
    });
}
