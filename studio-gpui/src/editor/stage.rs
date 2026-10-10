#![allow(dead_code)]
use gpui_kit::Context;

/// Represents the execution state of a project pipeline stage.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum StageState {
    /// The stage has not been executed or its previous result is invalid.
    #[default]
    Stale,

    /// The stage is ready to execute.
    Ready,

    /// The stage is currently executing.
    ///
    /// Contains a human-readable progress message.
    Busy(String),

    /// The stage failed to execute.
    ///
    /// Contains a human-readable error message.
    Failed(String),

    /// The stage completed successfully.
    ///
    /// Contains a human-readable success message.
    Succeeded(String),
}

impl StageState {
    /// Creates a busy state with a progress message.
    pub fn busy(message: impl Into<String>) -> Self {
        Self::Busy(message.into())
    }

    /// Creates a failed state with an error message.
    pub fn failed(message: impl Into<String>) -> Self {
        Self::Failed(message.into())
    }

    /// Creates a successful state with a completion message.
    pub fn succeeded(message: impl Into<String>) -> Self {
        Self::Succeeded(message.into())
    }

    /// Returns `true` if the stage is stale.
    pub fn is_stale(&self) -> bool {
        matches!(self, Self::Stale)
    }

    /// Returns `true` if the stage is ready to execute.
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns `true` if the stage is currently executing.
    pub fn is_busy(&self) -> bool {
        matches!(self, Self::Busy(_))
    }

    /// Returns `true` if the stage failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    /// Returns `true` if the stage completed successfully.
    pub fn is_succeeded(&self) -> bool {
        matches!(self, Self::Succeeded(_))
    }

    /// Returns `true` if the stage is executing or otherwise in progress.
    pub fn is_in_progress(&self) -> bool {
        self.is_busy()
    }

    /// Returns `true` if the stage has reached a terminal execution state.
    ///
    /// A stage is terminal when it has succeeded or failed.
    pub fn is_terminal(&self) -> bool {
        self.is_succeeded() || self.is_failed()
    }

    /// Returns `true` if the stage completed successfully or failed.
    ///
    /// Unlike [`Self::is_terminal`], this name is intended for callers
    /// interested in whether execution has finished.
    pub fn is_finished(&self) -> bool {
        self.is_terminal()
    }

    /// Returns `true` if the stage has a result from a completed execution.
    pub fn has_result(&self) -> bool {
        self.is_terminal()
    }

    /// Returns the stage's progress message, if it is busy.
    pub fn progress_message(&self) -> Option<&str> {
        match self {
            Self::Busy(message) => Some(message),
            _ => None,
        }
    }

    /// Returns the stage's error message, if it failed.
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Failed(message) => Some(message),
            _ => None,
        }
    }

    /// Returns the stage's success message, if it succeeded.
    pub fn success_message(&self) -> Option<&str> {
        match self {
            Self::Succeeded(message) => Some(message),
            _ => None,
        }
    }

    /// Returns the message associated with this state, if any.
    ///
    /// Returns a progress, error, or success message depending on the state.
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Busy(message) | Self::Failed(message) | Self::Succeeded(message) => Some(message),
            Self::Stale | Self::Ready => None,
        }
    }
}

/// Common state management for a pipeline stage.
pub trait StageView: Sized + 'static {
    /// Returns the current stage state.
    fn state(&self) -> &StageState;

    /// Returns the default message for the busy state.
    fn busy_message() -> &'static str;

    /// Returns the default message for the successful state.
    fn success_message() -> &'static str;

    /// Stores the new state.
    fn update_state(&mut self, state: StageState);

    /// Updates the stage state and notifies the view when it changes.
    fn set_state(&mut self, state: StageState, cx: &mut Context<Self>) {
        if self.state() == &state {
            return;
        }

        self.update_state(state);
        cx.notify();
    }

    /// Marks the stage as ready.
    fn mark_ready(&mut self, cx: &mut Context<Self>) {
        self.set_state(StageState::Ready, cx);
    }

    /// Marks the stage as stale.
    fn mark_stale(&mut self, cx: &mut Context<Self>) {
        self.set_state(StageState::Stale, cx);
    }

    /// Marks the stage as busy.
    fn mark_busy(&mut self, cx: &mut Context<Self>) {
        self.set_state(StageState::Busy(Self::busy_message().into()), cx);
    }

    /// Marks the stage as failed.
    fn mark_failed(&mut self, message: impl Into<String>, cx: &mut Context<Self>) {
        self.set_state(StageState::Failed(message.into()), cx);
    }

    /// Marks the stage as successful.
    fn mark_success(&mut self, cx: &mut Context<Self>) {
        self.set_state(StageState::Succeeded(Self::success_message().into()), cx);
    }
}
