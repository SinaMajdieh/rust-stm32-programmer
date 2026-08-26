use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use firmware_targets::{
    programmer::OpenOcd,
    stm32f103c8::{Hal, ProjectTemplate, Target},
};
use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};
use serde::Deserialize;

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Generate, build, and program STM32 firmware using a local LLM"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate main.c using an Ollama model.
    Generate {
        /// Ollama model to use.
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

    /// Build and program an existing generated project.
    Program {
        /// Name of the firmware project.
        firmware: String,
    },

    /// Generate, build, and program firmware.
    Run {
        /// Ollama model to use.
        model: String,

        /// Name of the firmware project.
        project: String,

        /// Description of the firmware to generate.
        prompt: Vec<String>,
    },
}

impl Command {
    pub fn project(&self) -> &str {
        match self {
            Self::Generate { project, .. }
            | Self::Build { project }
            | Self::Program { firmware: project }
            | Self::Run { project, .. } => project,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ollama: OllamaConfig,
    pub generation: GenerationConfig,

    #[serde(skip)]
    pub system_prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
    pub system_prompt_path: PathBuf,
    pub keep_alive: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct GenerationConfig {
    pub seed: u64,
    pub temperature: f32,
    pub context_length: u32,
    pub max_output_tokens: u32,
}

impl Config {
    pub fn load() -> Result<Self> {
        let contents = fs::read_to_string(CONFIG_PATH)
            .with_context(|| format!("failed to read {CONFIG_PATH}"))?;

        let mut config: Self = toml::from_str(&contents).context("failed to parse config.toml")?;

        config.system_prompt =
            fs::read_to_string(&config.ollama.system_prompt_path).with_context(|| {
                format!(
                    "failed to read system prompt: {}",
                    config.ollama.system_prompt_path.display()
                )
            })?;

        Ok(config)
    }
}

pub async fn generate_code(config: &Config, model: &str, prompt: &[String]) -> Result<String> {
    let prompt = prompt.join(" ");

    if prompt.trim().is_empty() {
        anyhow::bail!("prompt cannot be empty");
    }

    let client = OllamaClient::new(&config.ollama.url).context("failed to create Ollama client")?;

    let options = GenerateOptions::new()
        .with_seed(config.generation.seed)
        .with_temperature(config.generation.temperature)
        .with_context_length(config.generation.context_length)
        .with_maximum_output_tokens(config.generation.max_output_tokens);

    let request = GenerateRequest::new(model, prompt)
        .with_system_prompt(&config.system_prompt)
        .with_thinking(false)
        .with_keep_alive(&config.ollama.keep_alive)
        .with_options(options);

    let generation = client
        .generate(&request, Duration::from_secs(config.ollama.timeout_seconds))
        .await
        .context("code generation failed")?;

    println!(
        "Generated {} tokens at {:.1} tokens/s.",
        generation.generated_tokens,
        generation.tokens_per_second().unwrap_or(0.0),
    );

    Ok(unfence_code(&generation.response).to_owned())
}

pub fn save_source(project: &str, code: &str) -> Result<()> {
    let directory = Path::new(project);

    fs::create_dir_all(directory)
        .with_context(|| format!("failed to create project directory: {project}"))?;

    let path = directory.join("main.c");

    fs::write(&path, code).with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

pub fn build_project(project: &str) -> Result<firmware_targets::BuildArtifacts> {
    let directory = Path::new(project);
    let source_path = directory.join("main.c");

    let code = fs::read_to_string(&source_path)
        .with_context(|| format!("failed to read {}", source_path.display()))?;

    fs::remove_dir_all(directory)
        .with_context(|| format!("failed to remove existing project directory: {project}"))?;

    let mut project = Hal::generate(project).context("failed to create firmware project")?;

    project
        .add_source("main.c", &code)
        .context("failed to add main.c to firmware project")?;

    project.compile().context("firmware build failed")
}

pub fn program(firmware: impl AsRef<Path>) -> Result<firmware_targets::programmer::ProgramResult> {
    let target = Target::<OpenOcd>::default();

    target.program(firmware).context("programming failed")
}

pub fn unfence_code(code: &str) -> &str {
    let code = code.trim();

    let Some(code) = code.strip_prefix("```") else {
        return code;
    };

    let code = code
        .strip_prefix("c\n")
        .or_else(|| code.strip_prefix("C\n"))
        .or_else(|| code.strip_prefix('\n'))
        .unwrap_or(code);

    code.strip_suffix("```").unwrap_or(code).trim()
}
