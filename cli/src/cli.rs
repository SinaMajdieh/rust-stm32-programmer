use clap::{Parser, Subcommand};
use firmware_targets::{TargetKind, TemplateKind};

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

        /// Project directory where the generated source is saved.
        project: String,

        /// Prompt passed to the model.
        prompt: Vec<String>,
    },

    /// Build an existing project.
    Build {
        /// Target to use instead of the configured selected target.
        #[arg(long)]
        target: Option<TargetKind>,

        /// Template to use instead of the configured selected template.
        #[arg(long)]
        template: Option<TemplateKind>,

        /// Project directory containing the generated source.
        project: String,
    },

    /// Program an existing firmware binary.
    Program {
        /// Target to use instead of the configured selected target.
        #[arg(long)]
        target: Option<TargetKind>,

        /// Firmware ELF file to program.
        firmware: String,
    },

    /// Generate, build, and program firmware.
    Run {
        /// Model ID to use instead of the configured selected model.
        #[arg(long)]
        model: Option<String>,

        /// Target to use instead of the configured selected target.
        #[arg(long)]
        target: Option<TargetKind>,

        /// Template to use instead of the configured selected template.
        #[arg(long)]
        template: Option<TemplateKind>,

        /// Project directory where the generated source is saved.
        project: String,

        /// Prompt passed to the model.
        prompt: Vec<String>,
    },
}
