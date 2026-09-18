mod navigation;
// mod pipeline;
mod projects;

use gpui_kit::{
    AnyView, Context, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    component::Root, div, prelude::FluentBuilder,
};
use gpui_navigation::{RouteSegment, Router, RouterHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RouteSegment)]
pub enum WorkspaceRoute {
    Projects,
}

pub struct Workspace {
    views: Vec<ViewEntry>,
}

struct ViewEntry {
    view: AnyView,
    _subscription: Option<Subscription>,
}

impl Workspace {
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .when_some(self.views.last(), |this, entry| {
                this.child(entry.view.clone())
            })
            .children(Root::render_dialog_layer(window, cx))
    }
}

impl Router for Workspace {
    type Route = WorkspaceRoute;
    fn route(
        &mut self,
        route: &Self::Route,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<RouterHandle> {
        match route {
            WorkspaceRoute::Projects => Some(self.open_projects(window, cx).into()),
        }
    }
}
