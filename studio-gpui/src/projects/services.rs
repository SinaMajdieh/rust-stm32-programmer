use backend::project::Project;
use gpui_kit::{App, Window};
use gpui_navigation::{NavPath, Navigator};

use crate::{alert::show_alert, projects::ProjectsRoute, workspace::WorkspaceRoute};

pub(super) fn create_project(project: &Project, window: &mut Window, cx: &mut App) {
    match project.save() {
        Ok(_) => {
            println!("Project created: {:#?}", project);
            Navigator::go(
                NavPath::root(WorkspaceRoute::Projects).push(ProjectsRoute::Home),
                window,
                cx,
            );
        }
        Err(error) => show_alert(
            window,
            cx,
            "Faild to creat Project",
            error.user_message(),
            Some(error.to_string()),
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
                            error.user_message(),
                            Some(error.to_string()),
                        );
                    });
                }
            }
        })
        .detach();
}
