use clap::{Parser, Subcommand, ValueEnum};

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
    /// Generate `main.c` using an LLM.
    Generate {
        /// LLM provider to use.
        #[arg(long, value_enum, default_value_t = Provider::Ollama)]
        provider: Provider,

        /// Model to use.
        model: String,

        /// Name of the firmware project.
        project: String,

        /// Description of the firmware to generate.
        prompt: Vec<String>,
    },

    /// Build an existing generated project.
    Build {
        /// Name of the firmware project.
        project: String,
    },

    /// Program an existing firmware binary.
    Program {
        /// Path to the firmware binary.
        firmware: String,
    },

    /// Generate, build, and program firmware.
    Run {
        /// LLM provider to use.
        #[arg(long, value_enum, default_value_t = Provider::Ollama)]
        provider: Provider,

        /// Model to use.
        model: String,

        /// Name of the firmware project.
        project: String,

        /// Description of the firmware to generate.
        prompt: Vec<String>,
    },
}

/// LLM providers supported by the application.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Provider {
    /// Use a local Ollama server.
    Ollama,

    /// Use an OpenAI-compatible API.
    #[value(name = "openai")]
    OpenAi,
}
