use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    component::Root, div,
};

use crate::workspace::Workspace;

pub struct Studio {
    workspace: Entity<Workspace>,
}

impl Studio {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(Workspace::new);
        // let _ = Navigator::install(&workspace, cx);
        // Navigator::go(nav_path![WorkspaceRoute::Editor], window, cx);
        Self { workspace }
    }
}

impl Render for Studio {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family("Inter")
            .size_full()
            .child(self.workspace.clone())
            .children(Root::render_dialog_layer(window, cx))
    }
}
