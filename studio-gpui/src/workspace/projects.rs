use gpui_kit::{AppContext, Context, Window};

use crate::project::NewProjectView;

use super::Workspace;

impl Workspace {
    pub(super) fn open_new_project(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.new(|cx| NewProjectView::new(window, cx));

        let subscription = cx.subscribe_in(&view, window, |workspace, _view, event, window, cx| {
            workspace.handle_project_event(event, window, cx);
        });

        self.activate(view, Some(subscription), cx);
    }

    pub(super) fn create_project(
        &mut self,
        project: &crate::project::Project,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        println!("Create project: {project:?}");

        // Eventually:
        //
        // ProjectManager::create(project);
        //
        // Then:
        //
        // self.open_project(...);
    }

    pub(super) fn open_project(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        // ProjectManager::open(...)
        //
        // Eventually:
        //
        // self.activate(...);
    }
}
