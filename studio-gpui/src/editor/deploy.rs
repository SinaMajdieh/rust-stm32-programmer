use gpui_kit::{
    Context, EventEmitter, IntoElement, ParentElement, Render, Styled, Window,
    base::{Disableable, StyledExt},
    component::{
        alert::Alert,
        button::{Button, ButtonVariants},
        shimmer::ShimmerText,
    },
    div,
};

use crate::editor::stage::{StageState, StageView};

/// Events emitted by the deploy stage.
pub enum DeployEvent {
    /// Requests deployment of the current build artifact.
    Submit,
}

/// UI component for the project's deployment stage.
///
/// The parent controller owns the deployment operation and updates the state
/// when deployment starts, succeeds, or fails.
pub struct Deploy {
    state: StageState,
}

impl EventEmitter<DeployEvent> for Deploy {}

impl StageView for Deploy {
    fn state(&self) -> &StageState {
        &self.state
    }

    fn update_state(&mut self, state: StageState) {
        self.state = state;
    }

    fn busy_message() -> &'static str {
        "Deploying firmware..."
    }

    fn success_message() -> &'static str {
        "Firmware deployed successfully."
    }
}

impl Deploy {
    /// Creates a deploy stage in its initial state.
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            state: StageState::Stale,
        }
    }

    /// Returns whether deployment can be started.
    fn can_deploy(&self) -> bool {
        !self.state.is_busy()
    }

    /// Emits a request to deploy the current build artifact.
    fn deploy(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_deploy() {
            return;
        }

        self.mark_busy(cx);
        cx.emit(DeployEvent::Submit);
    }

    /// Renders the current deployment status, if it has a visible message.
    fn render_status(&self) -> Option<impl IntoElement> {
        match &self.state {
            StageState::Busy(message) => Some(
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .child(ShimmerText::new(message.clone()).text_lg().font_medium())
                    .into_any_element(),
            ),

            StageState::Succeeded(message) => Some(
                div()
                    .w_full()
                    .child(
                        Alert::success("deploy-success", message.clone())
                            .title("Deployment Completed"),
                    )
                    .into_any_element(),
            ),

            StageState::Failed(message) => Some(
                div()
                    .w_full()
                    .child(Alert::error("deploy-error", message.clone()).title("Deployment Failed"))
                    .into_any_element(),
            ),

            StageState::Stale | StageState::Ready => None,
        }
    }
}

impl Render for Deploy {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_deploy = self.can_deploy();

        div().size_full().v_flex().p_4().child(
            div()
                .w_full()
                .max_w_5_6()
                .v_flex()
                .gap_4()
                .child(
                    Button::new("deploy")
                        .primary()
                        .label("Deploy")
                        .disabled(!can_deploy)
                        .rounded_full()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.deploy(window, cx);
                        })),
                )
                .children(self.render_status()),
        )
    }
}
