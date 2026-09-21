use backend::project::Project;
use gpui_kit::{App, Window};
use gpui_navigation::{Navigator, nav_path};

use crate::{alert::show_alert, projects::ProjectsRoute, workspace::WorkspaceRoute};

pub(super) fn create_project(project: &Project, window: &mut Window, cx: &mut App) {
    match project.create() {
        Ok(_) => {
            println!("Project created: {:#?}", project);
            Navigator::go(
                nav_path![WorkspaceRoute::Projects, ProjectsRoute::Home],
                window,
                cx,
            );
        }
        Err(error) => show_alert(
            window,
            cx,
            "Faild to creat Project",
            error.to_string(),
            None,
        ),
    }
}

pub fn open_project(window: &mut Window, cx: &mut App) {
    window
        .spawn(cx, async move |cx| {
            let Some(path) = rfd::AsyncFileDialog::new()
                .set_title("Choose project location")
                .pick_folder()
                .await
            else {
                return;
            };

            let path = path.path().join("Project.toml");

            match Project::open(path) {
                Ok(project) => {
                    println!("Project opened: {project:#?}");
                }
                Err(error) => {
                    let _ = cx.update(|window, cx| {
                        show_alert(
                            window,
                            cx,
                            "Failed to open Project",
                            error.to_string(),
                            None,
                        );
                    });
                }
            }
        })
        .detach();
}
