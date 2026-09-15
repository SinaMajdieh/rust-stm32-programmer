mod events;
mod navigation;
mod projects;
mod welcome;

use gpui_kit::{
    AnyView, Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div,
    prelude::FluentBuilder,
};

pub struct Workspace {
    views: Vec<ViewEntry>,
}

struct ViewEntry {
    view: AnyView,
    _subscription: Option<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut workspace = Self { views: Vec::new() };

        workspace.open_welcome(window, cx);

        workspace
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .when_some(self.views.last(), |this, entry| {
                this.child(entry.view.clone())
            })
    }
}
