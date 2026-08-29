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
        #[arg(long, value_enum, default_value_t = Provider::Ollama)]
        provider: Provider,
        model: String,
        project: String,
        prompt: Vec<String>,
    },

    /// Build an existing generated project.
    Build { project: String },

    /// Program an existing firmware binary.
    Program { firmware: String },

    /// Generate, build, and program firmware.
    Run {
        #[arg(long, value_enum, default_value_t = Provider::Ollama)]
        provider: Provider,
        model: String,
        project: String,
        prompt: Vec<String>,
    },
}

/// LLM providers accepted by the CLI.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Provider {
    /// Use a local Ollama server.
    Ollama,

    /// Use an OpenAI-compatible API.
    #[value(name = "openai")]
    OpenAi,
}

impl From<Provider> for firmware_core::Provider {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::Ollama => Self::Ollama,
            Provider::OpenAi => Self::OpenAi,
        }
    }
}
