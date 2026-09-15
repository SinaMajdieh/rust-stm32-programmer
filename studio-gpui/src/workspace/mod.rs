mod events;
mod home;
mod navigation;
mod projects;

use gpui_kit::{
    AnyView, AppContext, Context, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    component::{Root, WindowExt},
    div,
    prelude::FluentBuilder,
};

use crate::alert::AlertContent;

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

        workspace.open_home(window, cx);

        workspace
    }

    pub fn show_alert(
        window: &mut Window,
        cx: &mut Context<Self>,
        title: impl Into<String>,
        message: impl Into<String>,
        details: Option<String>,
    ) {
        let title = title.into();
        let message = message.into();

        let content = cx.new(|_| AlertContent::new(message, details));

        window.open_alert_dialog(cx, move |alert, _, _| {
            let title = title.clone();
            let content = content.clone();

            alert
                .title(title)
                .content(move |dialog, _, _| dialog.child(content.clone()))
                .on_ok(|_, _, _| true)
        });
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
