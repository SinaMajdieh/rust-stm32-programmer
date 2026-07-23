use slint::ComponentHandle;
use crate::{programming, ui, MainWindow};

pub fn run() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    ui::initialize(&window);
    connect_file_picker(&window);
    connect_program_button(&window);

    window.run()
}

fn connect_file_picker(window: &MainWindow) {
    let window_weak = window.as_weak();

    window.on_browse_file(move || {
        let Some(path) = rfd::FileDialog::new()
            .set_title("Choose a program file")
            .add_filter("Program files", &["hex", "bin", "txt"])
            .pick_file()
        else {
            return;
        };

        let Some(window) = window_weak.upgrade() else {
            return;
        };

        window.set_selected_file_path(
            path.to_string_lossy().into_owned().into(),
        );

        ui::reset(&window);
    });
}

fn connect_program_button(window: &MainWindow) {
    let window_weak = window.as_weak();

    window.on_program_file(move || {
        let Some(window) = window_weak.upgrade() else {
            return;
        };

        let file_path = window.get_selected_file_path().to_string();

        ui::begin_programming(&window);
        programming::start(window.as_weak(), file_path);
    });
}
