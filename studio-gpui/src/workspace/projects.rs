use backend::project::Project;
use gpui_kit::{AppContext, Context, Window};

use crate::home::NewProject;

use super::Workspace;

impl Workspace {
    pub(super) fn open_new_project(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.new(|cx| NewProject::new(window, cx));

        let subscription = cx.subscribe_in(&view, window, |workspace, _view, event, window, cx| {
            workspace.handle_project_event(event, window, cx);
        });

        self.activate(view, Some(subscription), cx);
    }

    pub(super) fn create_project(
        &mut self,
        project: &Project,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match project.save() {
            Ok(_) => {
                println!("Project created: {:#?}", project);
                self.open_home(window, cx);
            }
            Err(error) => Self::show_alert(
                window,
                cx,
                "Faild to creat Project",
                error.user_message(),
                Some(error.to_string()),
            ),
        }

        // Eventually:
        //
        // ProjectManager::create(project);
        //
        // Then:
        //
        // self.open_project(...);
    }

    pub(super) fn open_project(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = rfd::FileDialog::new()
            .set_title("Choose project location")
            .pick_folder()
        else {
            return;
        };
        let path = path.join("Project.toml");
        match Project::open(path) {
            Ok(project) => println!("Project opened: {:#?}", project),
            Err(error) => Self::show_alert(
                window,
                cx,
                "Faild to open Project",
                error.user_message(),
                Some(error.to_string()),
            ),
        }
        // ProjectManager::open(...)
        //
        // Eventually:
        //
        // self.activate(...);
    }
}
