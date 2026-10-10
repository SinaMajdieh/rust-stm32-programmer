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

/// Events emitted by the build stage.
pub enum BuildEvent {
    /// Requests compilation of the current project.
    Submit,
}

/// UI component for the project's build stage.
///
/// The parent controller owns the build operation and updates the stage state
/// when compilation starts, succeeds, or fails.
pub struct Build {
    state: StageState,
}

impl EventEmitter<BuildEvent> for Build {}

impl StageView for Build {
    fn state(&self) -> &StageState {
        &self.state
    }

    fn update_state(&mut self, state: StageState) {
        self.state = state;
    }

    fn busy_message() -> &'static str {
        "Compiling firmware..."
    }

    fn success_message() -> &'static str {
        "Project compiled successfully."
    }
}

impl Build {
    /// Creates a build stage in its initial state.
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            state: StageState::Stale,
        }
    }

    /// Returns whether the build stage can be executed in its current state.
    ///
    /// A successful build must be invalidated before another build can run.
    fn can_run(&self) -> bool {
        !self.state.is_busy()
    }

    /// Emits a request to build the current project.
    fn build(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_run() {
            return;
        }

        self.mark_busy(cx);
        cx.emit(BuildEvent::Submit);
    }

    /// Renders the current build status, if it has a visible message.
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
                        Alert::success("build-success", message.clone()).title("Build Completed"),
                    )
                    .into_any_element(),
            ),

            StageState::Failed(error) => Some(
                div()
                    .w_full()
                    .child(Alert::error("build-error", error.clone()).title("Build Failed"))
                    .into_any_element(),
            ),

            StageState::Stale | StageState::Ready => None,
        }
    }
}

impl Render for Build {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_run = self.can_run();

        div().size_full().v_flex().p_4().child(
            div()
                .w_full()
                .max_w_5_6()
                .v_flex()
                .gap_4()
                .child(
                    Button::new("build")
                        .primary()
                        .label("Build")
                        .disabled(!can_run)
                        .rounded_full()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.build(window, cx);
                        })),
                )
                .children(self.render_status()),
        )
    }
}
