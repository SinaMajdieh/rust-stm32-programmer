use gpui_kit::{AppContext, Context, Entity, Window};

use crate::projects::ProjectsView;

use super::Workspace;

impl Workspace {
    pub(super) fn open_projects(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ProjectsView> {
        let view = cx.new(|cx| ProjectsView::new(window, cx));
        self.activate(view.clone(), None, cx);
        view
    }
}
