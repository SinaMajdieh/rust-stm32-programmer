use std::path::PathBuf;

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
        self.attach_open_project(&app);
        self.attach_open_recent_project(&app);
        self.attach_open(&app);
    }

    fn attach_new_project(&self, app: &AppWindow) {
        let navigator = self.navigator.clone();

        app.global::<ProjectAction>().on_new_project(move || {
            todo_log!("Implement creating a new project\nOpening generation screen.");
            navigator.go_to(Screen::Generation);
        });
    }

    fn attach_open_project(&self, app: &AppWindow) {
        app.global::<ProjectAction>().on_open_project(|| {
            todo_log!("Implement opening an existing project");
        });
    }

    fn attach_open_recent_project(&self, app: &AppWindow) {
        app.global::<ProjectAction>()
            .on_open_recent_project(|path| {
                let path = PathBuf::from(path.to_string());

                todo_log!("Implement opening recent project: {}", path.display());
            });
    }

    fn attach_open(&self, app: &AppWindow) {
        let navigator = self.navigator.clone();

        app.global::<ProjectAction>().on_open(move || {
            navigator.go_to(Screen::Project);
        });
    }
}
