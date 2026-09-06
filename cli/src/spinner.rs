use std::{
    io::{self, Write},
    time::Duration,
};

use tokio::{task::JoinHandle, time::sleep};

/// Configuration for a [`Spinner`].
#[derive(Debug, Clone)]
pub struct SpinnerConfig {
    /// Frames displayed by the spinner.
    pub frames: &'static [&'static str],

    /// Duration between frames.
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
/// The spinner stops automatically when dropped.
pub struct Spinner {
    task: JoinHandle<()>,
}

impl Spinner {
    /// Starts displaying a spinner with the given message
    /// using the default configuration.
    pub fn start(message: impl Into<String>) -> Self {
        Self::start_with_config(message, SpinnerConfig::default())
    }

    /// Starts displaying a spinner with the given message and configuration.
    pub fn start_with_config(message: impl Into<String>, config: SpinnerConfig) -> Self {
        let message = message.into();

        print!("\x1b[?25l"); // Hide cursor
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

        print!("\r\x1b[2K\x1b[?25h"); // Clear line + show cursor
        io::stdout().flush().ok();
    }
}
