mod cli;
mod config;

use std::{error::Error as StdError, process::ExitCode, time::Instant};

use backend::{
    Error, GenerationError, GenerationOutput, GenerationRequest, LlmGenerator, build_project,
    program, save_source,
};
use clap::Parser;

use cli::{Cli, Command};
use config::Config;

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

async fn run() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Generate {
            model,
            project,
            prompt,
        } => generate(&project, model.as_deref(), &prompt).await,

        Command::Build { project } => build(&project),

        Command::Program { firmware } => program_firmware(&firmware),

        Command::Run {
            model,
            project,
            prompt,
        } => {
            generate(&project, model.as_deref(), &prompt).await?;

            let start = Instant::now();
            let artifacts = build_project(&project)?;

            println!("Build finished in {} ms.", start.elapsed().as_millis());

            let start = Instant::now();
            program(artifacts.elf())?;

            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );

            Ok(())
        }
    }
}

/// Generates firmware source code using the configured or overridden model.
async fn generate(
    project: &str,
    model: Option<&str>,
    prompt_parts: &[String],
) -> Result<(), Error> {
    let config = Config::load(CONFIG_PATH).map_err(GenerationError::Config)?;
    let llm = config.llm;

    let model = model.unwrap_or(llm.selected_model.as_str());
    let system_prompt = llm.system_prompt().map_err(GenerationError::Config)?;
    let generator = LlmGenerator::from_config(llm.generator)?;

    let prompt = prompt_parts.join(" ");
    let request = GenerationRequest::new(model, &prompt, Some(&system_prompt));

    let output = generator.generate(request).await?;

    print_generation(&output);
    save_source(project, &output.code)?;

    Ok(())
}

/// Builds an existing project.
fn build(project: &str) -> Result<(), Error> {
    let start = Instant::now();

    build_project(project)?;

    println!("Build finished in {} ms.", start.elapsed().as_millis());

    Ok(())
}

/// Programs an existing firmware ELF.
fn program_firmware(firmware: &str) -> Result<(), Error> {
    let start = Instant::now();

    program(firmware)?;

    println!(
        "Programming finished in {} ms.",
        start.elapsed().as_millis()
    );

    Ok(())
}

/// Prints generated code and generation statistics.
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

/// Prints an error and its source chain.
fn print_error(error: &Error) {
    eprintln!("error: {error}");

    let mut source = StdError::source(error);

    while let Some(error) = source {
        eprintln!("caused by: {error}");
        source = StdError::source(error);
    }
}
