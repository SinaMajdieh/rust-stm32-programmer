mod cli;

use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;
use firmware_core::{
    Config, Error, GenerationOutput, build_project, generate_code, program, save_source,
};

use cli::{Cli, Command};

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
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Command::Generate {
            provider,
            model,
            project,
            prompt,
        } => {
            let output = generate_code(&config, provider.into(), &model, &prompt).await?;

            print_generation(&output);
            save_source(&project, &output.code)?;
        }

        Command::Build { project } => {
            run_build(&project)?;
        }

        Command::Program { firmware } => {
            run_program(&firmware)?;
        }

        Command::Run {
            provider,
            model,
            project,
            prompt,
        } => {
            let output = generate_code(&config, provider.into(), &model, &prompt).await?;

            print_generation(&output);
            save_source(&project, &output.code)?;

            let start = Instant::now();
            let artifacts = build_project(&project)?;

            println!("Build finished in {} ms.", start.elapsed().as_millis());

            let start = Instant::now();
            program(artifacts.elf())?;

            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );
        }
    }

    Ok(())
}

fn run_build(project: &str) -> Result<(), Error> {
    let start = Instant::now();

    build_project(project)?;

    println!("Build finished in {} ms.", start.elapsed().as_millis());

    Ok(())
}

fn run_program(firmware: &str) -> Result<(), Error> {
    let start = Instant::now();

    program(firmware)?;

    println!(
        "Programming finished in {} ms.",
        start.elapsed().as_millis()
    );

    Ok(())
}

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

fn print_error(error: &Error) {
    eprintln!("error: {error}");

    let mut source = std::error::Error::source(error);

    while let Some(error) = source {
        eprintln!("caused by: {error}");
        source = std::error::Error::source(error);
    }
}
