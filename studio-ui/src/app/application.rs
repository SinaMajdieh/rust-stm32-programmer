use crate::{AppWindow, app::navigation::Navigator};

use super::projects::Projects;

pub struct Application {
    navigator: Navigator,
    projects: Projects,
}

impl Application {
    pub fn new(app: &AppWindow) -> Self {
        let navigator = Navigator::new(app);
        Self {
            navigator: navigator.clone(),
            projects: Projects::new(app, &navigator),
        }
    }

    pub fn attach(&self) {
        self.navigator.attach();
        self.projects.attach();
    }
}
