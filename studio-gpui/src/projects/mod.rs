mod home;
mod new_project;
mod services;

use gpui_kit::{AppContext, Context, IntoElement, Render, Styled, Window};
use gpui_navigation::Navigator;
pub use home::Home;
pub use new_project::NewProject;

pub struct Projects;

impl Projects {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let home = cx.new(|_| Home::new());
        Navigator::new().scope("projects").push(home, cx).unwrap();
        Self
    }
}

impl Render for Projects {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Navigator::new()
            .scope("projects")
            .stack(cx)
            .unwrap()
            .size_full()
            .overflow_hidden()
    }
}
