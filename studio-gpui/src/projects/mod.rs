mod home;
mod new_project;
mod services;

use gpui_kit::{
    AppContext, Context, Entity, IntoElement, Render, Styled, Window,
    base::{NavMotion, NavStack, NavStackState},
};
pub use home::Home;
pub use new_project::NewProject;

pub struct Projects {
    stack: Entity<NavStackState>,
}

impl Projects {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let stack = cx.new(|_| NavStackState::new());

        let home = cx.new(|_| Home::new(stack.downgrade()));

        stack.update(cx, |stack, cx| {
            stack.push(home, NavMotion::Immediate, cx);
        });

        Self { stack }
    }
}

impl Render for Projects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        NavStack::new(&self.stack)
            .size_full()
            .overflow_hidden()
            .item(|page, _, _| page.into_any_element())
    }
}
