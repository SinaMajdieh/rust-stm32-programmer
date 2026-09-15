pub fn create_project(
    // &mut self,
    project: &crate::project::Project,
    // _window: &mut Window,
    // _cx: &mut Context<Self>,
) {
    println!("Create project: {project:?}");

    // Eventually:
    //
    // ProjectManager::create(project);
    //
    // Then:
    //
    // self.open_project(...);
}

pub fn open_project() {
    let Some(path) = rfd::FileDialog::new()
        .set_title("Choose project location")
        .pick_folder()
    else {
        return;
    };
    println!("Open Project: {:?}", path);
}
