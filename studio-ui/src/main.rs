use slint::ComponentHandle;

mod generation;

slint::include_modules!();

macro_rules! todo_log {
    ($($arg:tt)*) => {
        println!("[TODO] {}", format_args!($($arg)*))
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    ui.on_new_project(|| todo_log!("Create New project"));
    ui.on_open_project(|| todo_log!("Open an existing project"));
    ui.on_open_recent_project(|path| todo_log!("Open recent project on {:#?}", path));

    ui.run()?;

    Ok(())
}
