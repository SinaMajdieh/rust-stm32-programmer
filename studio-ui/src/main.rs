mod app;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;

    app::Application::new(&app).attach();

    app.run()?;

    Ok(())
}
