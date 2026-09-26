use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window};
use gpui_navigation::Navigator;

use crate::projects::Projects;

pub struct Workspace;

impl Workspace {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let page = cx.new(Projects::new);
        Navigator::new().scope("workspace").push(page, cx).unwrap();
        Self
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Navigator::new()
            .scope("workspace")
            .stack(cx)
            .unwrap()
            .size_full()
            .overflow_hidden()
    }
}
