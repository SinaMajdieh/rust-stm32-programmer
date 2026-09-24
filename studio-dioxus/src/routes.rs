use dioxus::prelude::*;

use crate::{home::Home, new_project::NewProject};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/projects/new")]
    NewProject,
}
