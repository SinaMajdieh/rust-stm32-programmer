mod home;
mod services;
mod new_project;

pub use home::Home;
pub use new_project::NewProject;

use gpui_kit::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Window, div};
use gpui_navigation::{RouteSegment, Router, RouterHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RouteSegment)]
pub enum ProjectsRoute {
    Home,
    NewProject,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum View {
    #[default]
    Home,
    NewProject,
}

pub struct ProjectsView {
    home: Entity<Home>,
    new_project: Entity<NewProject>,
    active: View,
}

impl ProjectsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            home: cx.new(|_| Home::new()),
            new_project: cx.new(|cx| NewProject::new(window, cx)),
            active: View::default(),
        }
    }

    fn open_home(&mut self, cx: &mut Context<Self>) {
        self.active = View::Home;
        cx.notify();
    }

    fn open_new_project(&mut self, cx: &mut Context<Self>) {
        self.active = View::NewProject;
        cx.notify();
    }
}

impl Render for ProjectsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let view = match self.active {
            View::Home => self.home.clone().into_any_element(),
            View::NewProject => self.new_project.clone().into_any_element(),
        };

        div().child(view)
    }
}

impl Router for ProjectsView {
    type Route = ProjectsRoute;
    fn route(
        &mut self,
        route: &Self::Route,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<RouterHandle> {
        match route {
            ProjectsRoute::Home => {
                self.open_home(cx);
                None
            }
            ProjectsRoute::NewProject => {
                self.open_new_project(cx);
                None
            }
        }
    }
}
