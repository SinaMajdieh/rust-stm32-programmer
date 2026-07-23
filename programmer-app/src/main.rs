mod app;
mod programming;
mod ui;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}