use clap::{Parser, Subcommand};

/// Command-line interface for the firmware generation tool.
#[derive(Debug, Parser)]
#[command(
    version,
    about = "Generate, build, and program STM32 firmware using an LLM"
)]
pub struct Cli {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Commands supported by the firmware generation tool.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate firmware source code.
    Generate {
        /// Model ID to use instead of the configured selected model.
        #[arg(long)]
        model: Option<String>,

        /// Project directory.
        project: String,

        /// Prompt passed to the model.
        prompt: Vec<String>,
    },

    /// Build an existing project.
    Build {
        /// Project directory.
        project: String,
    },

    /// Program an existing firmware binary.
    Program {
        /// Firmware ELF file.
        firmware: String,
    },

    /// Generate, build, and program firmware.
    Run {
        /// Model ID to use instead of the configured selected model.
        #[arg(long)]
        model: Option<String>,

        /// Project directory.
        project: String,

        /// Prompt passed to the model.
        prompt: Vec<String>,
    },
}
