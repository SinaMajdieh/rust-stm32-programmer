use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window, div};
use gpui_navigation::Navigator;

use crate::projects::Projects;

#[derive(Debug, Clone, Copy)]
pub struct WorkspaceScope;

pub struct Workspace;

impl Workspace {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let page = cx.new(Projects::new);
        Navigator::new().scope(WorkspaceScope).push(page, cx);
        Self
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match Navigator::new().scope(WorkspaceScope).stack(cx) {
            Some(stack) => stack.size_full().overflow_hidden().into_any_element(),
            None => div().into_any_element(),
        }
    }
}
