use std::thread;

use programmer_core::program_file;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    setup_file_picker(&ui);
    setup_programmer(&ui);

    ui.run()
}

fn setup_file_picker(ui: &MainWindow) {
    let ui_weak = ui.as_weak();

    ui.on_browse_file(move || {
        let Some(path) = rfd::FileDialog::new()
            .set_title("Choose a text file")
            .add_filter("Text files", &["txt"])
            .pick_file()
        else {
            return;
        };

        let Some(ui) = ui_weak.upgrade() else {
            return;
        };

        ui.set_selected_file_path(
            path.to_string_lossy().into_owned().into()
        );

        ui.set_file_contents("".into());
        ui.set_progress_value(0.0);
    });
}

fn setup_programmer(ui: &MainWindow) {
    let ui_weak = ui.as_weak();

    ui.on_program_file(move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };

        let file_path = ui.get_selected_file_path().to_string();

        ui.set_file_contents("".into());
        ui.set_progress_value(0.0);
        ui.set_is_programming(true);

        let worker_ui = ui.as_weak();

        thread::spawn(move || {
            let result = program_file(file_path, |update| {
                let update_ui = worker_ui.clone();

                let _ = update_ui.upgrade_in_event_loop(move |ui| {
                    ui.set_progress_value(update.percentage as f32);

                    let mut contents = ui.get_file_contents().to_string();

                    contents.push_str(&update.text_chunk);

                    ui.set_file_contents(contents.into());
                });
            });

            let finish_ui = worker_ui.clone();

            let _ = finish_ui.upgrade_in_event_loop(move |ui| {
                ui.set_is_programming(false);

                if let Err(error) = result {
                    ui.set_file_contents(
                        format!("Programming failed:\n{error}").into(),
                    );
                }
            });
        });
    });
}