use backend::project::Project;
use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window, div};
use gpui_navigation::Navigator;

use crate::{editor::Editor, workspace::Workspace};

pub struct Studio;

impl Studio {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(Workspace::new);
        let editor = cx.new(|cx| Editor::new(Project::default(), _window, cx));
        Navigator::new().push(editor, cx);
        Self
    }
}

impl Render for Studio {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match Navigator::new().stack(cx) {
            Some(stack) => stack.size_full().overflow_hidden().into_any_element(),

            None => div().into_any_element(),
        }
    }
}
