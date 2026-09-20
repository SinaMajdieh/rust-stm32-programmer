use gpui_kit::{assets::IconName, base::StyledExt, component::ActiveTheme, prelude::*, *};
use gpui_navigation::{Navigator, nav_path};

use crate::{
    projects::{ProjectsRoute, services::open_project},
    workspace::WorkspaceRoute,
};

// pub enum HomeEvent {
//     NewProject,
//     OpenProject,
// }

pub struct Home;

// impl EventEmitter<HomeEvent> for Home {}

impl Home {
    pub fn new() -> Self {
        Self
    }
}

impl Render for Home {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .items_center()
            .w_full()
            .pt_12()
            .child(header(cx))
            .child(
                div()
                    .flex()
                    .gap_4()
                    .mt_8()
                    .child(create_project_card(cx))
                    .child(open_project_card(cx)),
            )
    }
}

fn header(cx: &mut Context<Home>) -> impl IntoElement {
    let theme = cx.theme();

    div()
        .v_flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .font_semibold()
                .text_3xl()
                .text_color(theme.foreground)
                .child("Welcome back"),
        )
        .child(
            div()
                .text_lg()
                .text_color(theme.muted_foreground)
                .child("What would you like to do?"),
        )
}

fn create_project_card(cx: &mut Context<Home>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .id("create-project")
        .v_flex()
        .items_center()
        .justify_center()
        .gap_1()
        .w_64()
        .h_48()
        .p_4()
        .rounded(theme.radius)
        .bg(theme.background)
        .border_1()
        .border_color(theme.border)
        .cursor_pointer()
        .hover(|this| this.bg(theme.muted))
        .on_click(cx.listener(|_, _, window, cx| {
            Navigator::go(
                nav_path![WorkspaceRoute::Projects, ProjectsRoute::NewProject],
                window,
                cx,
            );
        }))
        .child(div().text_2xl().child(IconName::Plus))
        .child(
            div()
                .font_semibold()
                .text_xl()
                .text_color(theme.foreground)
                .child("New Project"),
        )
        .child(
            div()
                .text_lg()
                .text_color(theme.muted_foreground)
                .child("Create a new project"),
        )
}

fn open_project_card(cx: &mut Context<Home>) -> impl IntoElement {
    let theme = cx.theme();

    div()
        .id("open-project")
        .v_flex()
        .items_center()
        .justify_center()
        .gap_1()
        .w_64()
        .h_48()
        .p_4()
        .rounded(theme.radius)
        .bg(theme.background)
        .border_1()
        .border_color(theme.border)
        .cursor_pointer()
        .hover(|this| this.bg(theme.muted))
        .on_click(cx.listener(|_, _, window, cx| {
            open_project(window, cx);
        }))
        .child(div().text_2xl().child(IconName::FolderOpen))
        .child(
            div()
                .font_semibold()
                .text_xl()
                .text_color(theme.foreground)
                .child("Open Project"),
        )
        .child(
            div()
                .text_lg()
                .text_color(theme.muted_foreground)
                .child("Open an existing project"),
        )
}
