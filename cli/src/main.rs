mod cli;
mod config;
mod firmware;
mod generation;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use config::Config;
use firmware::{build_project, program, save_source};
use generation::generate_code;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Command::Generate {
            provider,
            model,
            project,
            prompt,
        } => {
            let code = generate_code(&config, provider, &model, &prompt).await?;
            println!("Generated code: {code}");
            save_source(&project, &code)?;
        }

        Command::Build { project } => {
            let start = Instant::now();
            build_project(&project)?;
            println!("Build finished in {} ms.", start.elapsed().as_millis())
        }

        Command::Program { firmware } => {
            let start = Instant::now();
            program(&firmware)?;
            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );
        }

        Command::Run {
            provider,
            model,
            project,
            prompt,
        } => {
            let code = generate_code(&config, provider, &model, &prompt).await?;
            println!("Generated code:\n ```C\n{code}\n```");
            save_source(&project, &code)?;

            let start = Instant::now();
            let artifacts = build_project(&project)?;
            println!("Build finished in {} ms.", start.elapsed().as_millis());

            let start = Instant::now();
            program(&artifacts.elf())?;
            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );
        }
    }

    Ok(())
}
