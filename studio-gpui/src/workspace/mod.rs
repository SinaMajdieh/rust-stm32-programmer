use gpui_kit::{
    AppContext, Context, Entity, IntoElement, Render, Styled, Window,
    base::{NavMotion, NavStack, NavStackState},
};

use crate::projects::Projects;

pub struct Workspace {
    stack: Entity<NavStackState>,
}

impl Workspace {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let stack = cx.new(|_| NavStackState::new());

        let page = cx.new(Projects::new);

        stack.update(cx, |stack, cx| {
            stack.push(page, NavMotion::Immediate, cx);
        });

        Self { stack }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        NavStack::new(&self.stack)
            .size_full()
            .overflow_hidden()
            .item(|page, _, _| page.into_any_element())
    }
}
