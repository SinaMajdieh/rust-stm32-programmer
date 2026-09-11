use slint::{ComponentHandle, Weak};

use crate::{AppWindow, Navigation, Screen};

#[derive(Clone)]
pub struct Navigator {
    app: Weak<AppWindow>,
}

impl Navigator {
    pub fn new(app: &AppWindow) -> Self {
        Self { app: app.as_weak() }
    }

    pub fn attach(&self) {
        let Some(app) = self.app.upgrade() else {
            return;
        };

        let navigator = self.clone();

        app.global::<Navigation>().on_go_to_screen(move |screen| {
            navigator.go_to(screen);
        });
    }

    pub fn go_to(&self, screen: Screen) {
        let Some(app) = self.app.upgrade() else {
            return;
        };

        app.global::<Navigation>().set_current_screen(screen);
    }
}
