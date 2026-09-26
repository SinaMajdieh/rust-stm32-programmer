use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window};
use gpui_navigation::Navigator;

use crate::workspace::Workspace;

pub struct Studio;

impl Studio {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(Workspace::new);
        Navigator::new().push(workspace.clone(), cx).unwrap();
        Self
    }
}

impl Render for Studio {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Navigator::new()
            .stack(cx)
            .unwrap()
            .size_full()
            .overflow_hidden()
    }
}
