use gpui_kit::Global;
use tokio::runtime::Runtime;

/// Application-wide Tokio runtime.
///
/// GPUI provides its own async executor, but some dependencies used by the
/// application—such as Reqwest's async client—require Tokio. This type keeps
/// Tokio runtime management in one place while exposing only a small spawning
/// API to the rest of the application.
///
/// The runtime is stored as a GPUI global and lives for the lifetime of the
/// application.
pub struct TokioRuntime {
    runtime: Runtime,
}

impl TokioRuntime {
    /// Creates a new application Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if the Tokio runtime cannot be initialized.
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new().expect("failed to initialize Tokio runtime"),
        }
    }

    /// Returns a handle that can be used to spawn work on the application
    /// Tokio runtime.
    pub fn handle(&self) -> tokio::runtime::Handle {
        self.runtime.handle().clone()
    }
}

impl Default for TokioRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Global for TokioRuntime {}
