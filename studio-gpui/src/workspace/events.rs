use gpui_kit::{Context, Window};

use crate::home::{HomeEvent, NewProjectEvent};

use super::Workspace;

impl Workspace {
    pub(super) fn handle_welcome_event(
        &mut self,
        event: &HomeEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            HomeEvent::NewProject => {
                self.open_new_project(window, cx);
            }

            HomeEvent::OpenProject => {
                self.open_project(window, cx);
            }
        }
    }

    pub(super) fn handle_project_event(
        &mut self,
        event: &NewProjectEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NewProjectEvent::Cancel => {
                self.deactivate(cx);
            }

            NewProjectEvent::Create(project) => {
                self.create_project(project, window, cx);
            }
        }
    }
}
