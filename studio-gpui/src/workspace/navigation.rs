use gpui_kit::{Context, Entity, Render, Subscription};

use super::{ViewEntry, Workspace};

impl Workspace {
    pub(super) fn activate<T>(
        &mut self,
        view: Entity<T>,
        subscription: Option<Subscription>,
        cx: &mut Context<Self>,
    ) where
        T: Render,
    {
        self.views.push(ViewEntry {
            view: view.into(),
            _subscription: subscription,
        });

        cx.notify();
    }

    // pub(super) fn deactivate(&mut self, cx: &mut Context<Self>) {
    //     if self.views.len() > 1 {
    //         self.views.pop();
    //         cx.notify();
    //     }
    // }
}
