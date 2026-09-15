use gpui_kit::{AppContext, Context, Window};

use crate::welcome::Welcome;

use super::Workspace;

impl Workspace {
    pub(super) fn open_welcome(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.new(|_cx| Welcome::new());

        let subscription = cx.subscribe_in(&view, window, |workspace, _view, event, window, cx| {
            workspace.handle_welcome_event(event, window, cx);
        });

        self.activate(view, Some(subscription), cx);
    }
}
