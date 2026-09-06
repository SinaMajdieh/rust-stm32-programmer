//! Terminal spinner used to indicate progress during long-running operations.
//!
//! [`Spinner`] runs its animation in a background Tokio task and automatically
//! stops the animation when dropped.

use std::{
    io::{self, Write},
    time::Duration,
};

use tokio::{task::JoinHandle, time::sleep};

/// Configuration for a [`Spinner`].
///
/// A spinner cycles through [`Self::frames`] at [`Self::frame_interval`].
#[derive(Debug, Clone)]
pub struct SpinnerConfig {
    /// Frames displayed by the spinner.
    ///
    /// The frames are displayed in order and then repeated from the beginning.
    /// This slice must not be empty.
    pub frames: &'static [&'static str],

    /// Duration between consecutive frames.
    pub frame_interval: Duration,
}

impl Default for SpinnerConfig {
    fn default() -> Self {
        Self {
            frames: &["", " •", " ••", " •••"],
            frame_interval: Duration::from_millis(400),
        }
    }
}

/// Displays an animated terminal spinner while a command is running.
///
/// The spinner runs in a background Tokio task. Dropping the [`Spinner`]
/// aborts the task, clears the current terminal line, and restores the
/// terminal cursor.
pub struct Spinner {
    task: JoinHandle<()>,
}

impl Spinner {
    /// Starts displaying a spinner with the given message using the default
    /// [`SpinnerConfig`].
    pub fn start(message: impl Into<String>) -> Self {
        Self::start_with_config(message, SpinnerConfig::default())
    }

    /// Starts displaying a spinner with the given message and configuration.
    ///
    /// The terminal cursor is hidden while the spinner is running and restored
    /// when the spinner is dropped.
    ///
    /// # Panics
    ///
    /// Panics if [`SpinnerConfig::frames`] is empty.
    pub fn start_with_config(message: impl Into<String>, config: SpinnerConfig) -> Self {
        let message = message.into();

        print!("\x1b[?25l");
        io::stdout().flush().ok();

        let task = tokio::spawn(async move {
            let mut i = 0;

            loop {
                let frame = config.frames[i % config.frames.len()];

                print!("\r\x1b[2K{}{}", message, frame);
                io::stdout().flush().ok();

                i += 1;
                sleep(config.frame_interval).await;
            }
        });

        Self { task }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.task.abort();

        print!("\r\x1b[2K\x1b[?25h");
        io::stdout().flush().ok();
    }
}
