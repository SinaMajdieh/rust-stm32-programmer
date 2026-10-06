mod home;
mod new_project;
mod services;

use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window, div};
use gpui_navigation::Navigator;
pub use home::Home;
pub use new_project::NewProject;

#[derive(Debug, Clone, Copy)]
pub struct ProjectsScope;

pub struct Projects;

impl Projects {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let home = cx.new(|_| Home::new());
        Navigator::new().scope(ProjectsScope).push(home, cx);
        Self
    }
}

impl Render for Projects {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match Navigator::new().scope(ProjectsScope).stack(cx) {
            Some(stack) => stack.size_full().overflow_hidden().into_any_element(),
            None => div().into_any_element(),
        }
    }
}
