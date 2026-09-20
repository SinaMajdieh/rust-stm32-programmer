use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_navigation::{Navigator, nav_path};

use crate::workspace::{Workspace, WorkspaceRoute};

pub struct Studio {
    workspace: Entity<Workspace>,
}

impl Studio {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|_| Workspace::new());
        let _ = Navigator::install(&workspace, cx);
        Navigator::go(nav_path![WorkspaceRoute::Projects], window, cx);
        Self { workspace }
    }
}

impl Render for Studio {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family("Inter")
            .size_full()
            .child(self.workspace.clone())
    }
}
