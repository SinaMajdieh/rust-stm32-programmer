use std::ops::Deref;

use backend::project::Project;
use gpui_kit::{
    App, AppContext, WeakEntity, Window,
    base::{NavMotion, NavStackState},
};

use crate::{alert::show_alert, editor::Editor};

pub(super) fn create_project(
    project: Project,
    stack: WeakEntity<NavStackState>,
    window: &mut Window,
    cx: &mut App,
) {
    match project.create() {
        Ok(_) => {
            println!("Project created: {:#?}", project);
            let Some(stack) = stack.upgrade() else {
                return;
            };
            let editor = cx.new(|cx| Editor::new(project, cx));
            stack.update(cx, |stack, cx| {
                stack.replace(editor, NavMotion::Immediate, cx);
            });
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

pub fn open_project(stack: WeakEntity<NavStackState>, window: &mut Window, cx: &mut App) {
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
                    let Some(stack) = stack.upgrade() else {
                        return;
                    };
                    let editor = cx.new(|cx| Editor::new(project, cx));
                    stack.update(cx, |stack, cx| {
                        stack.replace(editor, NavMotion::Animated, cx);
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
