use backend::project::Project;
use gpui_kit::{App, AppContext, Window};
use gpui_navigation::Navigator;

use crate::{alert::show_alert, editor::Editor};

pub(super) fn create_project(project: Project, window: &mut Window, cx: &mut App) {
    match project.create() {
        Ok(_) => {
            println!("Project created: {:#?}", project);
            let editor = cx.new(|cx| Editor::new(project, cx));
            Navigator::new()
                .scope("workspace")
                .replace(editor, cx)
                .unwrap();
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
                    let editor = cx.new(|cx| Editor::new(project, cx));
                    let _ = cx.update(|_, cx| {
                        Navigator::new()
                            .scope("workspace")
                            .replace(editor, cx)
                            .unwrap();
                    });
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
