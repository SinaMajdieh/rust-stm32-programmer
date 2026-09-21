//! Command-line entry point for firmware generation, building, and programming.
//!
//! The CLI coordinates configuration loading, LLM-based source generation,
//! firmware project building, and target programming.

mod cli;
mod config;
mod spinner;

use std::{path::Path, process::ExitCode, time::Instant};

use backend::{
    Error, GenerationOutput, LlmGenerator,
    project::{GenerationRequest, Project, ProjectError},
};
use clap::Parser;
use firmware_targets::{TargetKind, TemplateKind};

use cli::{Cli, Command};
use config::Config;
use spinner::Spinner;

const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            print_error(&error);
            ExitCode::FAILURE
        }
    }
}

/// Executes the command selected by the user.
async fn run() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Generate {
            model,
            project,
            prompt,
        } => generate(&project, model.as_deref(), &prompt).await,

        Command::Build {
            target,
            template,
            project,
        } => build(&project, target, template).map(|_| ()),

        Command::Program { target, project } => program_firmware(target, project),

        Command::Run {
            model,
            target,
            template,
            project,
            prompt,
        } => {
            generate(&project, model.as_deref(), &prompt).await?;
            build(&project, target, template)?;
            program_firmware(target, &project)?;

            Ok(())
        }
    }
}

/// Resolves firmware settings using command-line overrides or the configuration.
///
/// A provided command-line value takes precedence over the corresponding
/// configured value.
fn resolve_firmware(
    target: Option<TargetKind>,
    template: Option<TemplateKind>,
) -> Result<(TargetKind, TemplateKind), Error> {
    let config = Config::load(CONFIG_PATH)?;
    let firmware = config.firmware;

    Ok((
        target.unwrap_or(firmware.selected_target),
        template.unwrap_or(firmware.generation.selected_template),
    ))
}

/// Generates firmware source code using the configured or overridden model.
///
/// The generated source is printed along with generation statistics and saved
/// as `main.c` in the specified project directory.
async fn generate(
    project: &str,
    model: Option<&str>,
    prompt_parts: &[String],
) -> Result<(), Error> {
    let config = Config::load(CONFIG_PATH)?;
    let llm = config.llm;

    let model = model.unwrap_or(llm.selected_model.as_str());
    let prompt = prompt_parts.join(" ");
    let system_prompt = llm.system_prompt()?;

    let generator = LlmGenerator::from_config(llm.generator)?;
    let request = GenerationRequest::new(model, prompt, Some(system_prompt));

    let mut project = Project::new().with_root(project).with_name(project);

    let output = {
        let _spinner = Spinner::start("Generating code");
        project.generate(request, &generator).await?;
        project.generation().map(|generation| generation.result())
    };

    project.create().map_err(ProjectError::Io)?;

    if let Some(output) = output {
        print_generation(output);
    }

    Ok(())
}

/// Builds an existing firmware project using the configured or overridden
/// target and template.
fn build(
    project: &str,
    target: Option<TargetKind>,
    template: Option<TemplateKind>,
) -> Result<(), Error> {
    let (target, template) = resolve_firmware(target, template)?;

    let start = Instant::now();
    let mut project = Project::open_from_dir(project)
        .map_err(ProjectError::Io)?
        .with_target(target)
        .with_template(template);

    {
        let _spinner = Spinner::start("Building");
        project.build()?;
    };

    println!("Build finished in {} ms.", start.elapsed().as_millis());
    project.save().map_err(ProjectError::Io)?;
    Ok(())
}

/// Programs an existing firmware ELF using the configured or overridden target.
fn program_firmware(target: Option<TargetKind>, project: impl AsRef<Path>) -> Result<(), Error> {
    let (target, _) = resolve_firmware(target, None)?;

    let start = Instant::now();
    let mut project = Project::open_from_dir(project)
        .map_err(ProjectError::Io)?
        .with_target(target);
    {
        let _spinner = Spinner::start("Programming");
        project.program()?;
    }

    println!(
        "Programming finished in {} ms.",
        start.elapsed().as_millis()
    );

    project.save().map_err(ProjectError::Io)?;

    Ok(())
}

/// Prints generated source code and generation statistics.
fn print_generation(output: &GenerationOutput) {
    let stats = &output.statistics;

    println!("Generated code:\n```c\n{}\n```", output.code);

    if let Some(prompt_tokens) = stats.prompt_tokens {
        println!("Prompt: {prompt_tokens} tokens.");
    }

    println!("Generated: {} tokens.", stats.generated_tokens);
    println!("Time: {:.2}s.", stats.elapsed.as_secs_f64());
    println!("Speed: {:.1} tokens/s.", stats.tokens_per_second());
}

/// Prints an error to standard error.
fn print_error(error: &Error) {
    eprintln!("Error: {error}");
}
