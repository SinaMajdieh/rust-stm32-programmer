use std::path::{Path, PathBuf};

use slint::{ComponentHandle, Weak};

use crate::{AppWindow, ProjectAction, Screen, app::navigation::Navigator, todo_log};

pub struct Projects {
    app: Weak<AppWindow>,
    navigator: Navigator,
}

impl Projects {
    pub fn new(app: &AppWindow, navigator: &Navigator) -> Self {
        Self {
            app: app.as_weak(),
            navigator: navigator.clone(),
        }
    }

    pub fn attach(&self) {
        let Some(app) = self.app.upgrade() else {
            return;
        };

        self.attach_new_project(&app);
        self.attach_open_project_dialogue(&app);
        self.attach_open_project(&app);
        self.attach_open(&app);
    }

    fn attach_new_project(&self, app: &AppWindow) {
        let navigator = self.navigator.clone();

        app.global::<ProjectAction>().on_new_project(move || {
            todo_log!("Implement creating a new project\nOpening generation screen.");
            navigator.go_to(Screen::Generation);
        });
    }

    fn attach_open_project_dialogue(&self, app: &AppWindow) {
        app.global::<ProjectAction>().on_open_project_dialogue(|| {
            let Some(path) = rfd::FileDialog::new()
                .set_title("Choose STM32 Studio Project")
                .pick_folder()
            else {
                return;
            };

            Self::open_project(path);
        });
    }

    fn attach_open_project(&self, app: &AppWindow) {
        app.global::<ProjectAction>().on_open_project(|path| {
            let path = PathBuf::from(path.to_string());
            Self::open_project(path);
        });
    }

    fn attach_open(&self, app: &AppWindow) {
        let navigator = self.navigator.clone();

        app.global::<ProjectAction>().on_open(move || {
            navigator.go_to(Screen::Project);
        });
    }
}

impl Projects {
    fn open_project(path: impl AsRef<Path>) {
        let project_dir = path.as_ref();
        todo_log!("Implement opening project: {}", project_dir.display());
    }
}
