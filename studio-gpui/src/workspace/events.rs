use gpui_kit::{Context, Window};

use crate::{project::FormEvent, welcome::WelcomeEvent};

use super::Workspace;

impl Workspace {
    pub(super) fn handle_welcome_event(
        &mut self,
        event: &WelcomeEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            WelcomeEvent::NewProject => {
                self.open_new_project(window, cx);
            }

            WelcomeEvent::OpenProject => {
                self.open_project(window, cx);
            }
        }
    }

    pub(super) fn handle_project_event(
        &mut self,
        event: &FormEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            FormEvent::Cancel => {
                self.deactivate(cx);
            }

            FormEvent::Create(project) => {
                self.create_project(project, window, cx);
            }
        }
    }
}
