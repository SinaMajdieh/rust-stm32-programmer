use std::time::Instant;

use clap::Parser;

use cli::{Cli, Command, Config, build_project, generate_code, program, save_source};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Generate {
            model,
            project,
            prompt,
        } => {
            let config = Config::load()?;

            println!("Generating {project} with {model}...");

            let code = generate_code(&config, &model, &prompt).await?;

            save_source(&project, &code)?;

            println!("Generated {project}/main.c.");
            println!("{code}");
        }

        Command::Build { project } => {
            let start = Instant::now();
            println!("Building {project}...");

            build_project(&project)?;

            println!("Build finished in {} ms.", start.elapsed().as_millis());
        }

        Command::Program { firmware } => {
            let start = Instant::now();
            println!("Programming {firmware}...");

            program(firmware)?;

            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );
        }

        Command::Run {
            model,
            project,
            prompt,
        } => {
            let config = Config::load()?;

            println!("Generating {project} with {model}...");
            let code = generate_code(&config, &model, &prompt).await?;
            save_source(&project, &code)?;
            println!("Generation complete.");

            let start = Instant::now();
            println!("Building {project}...");
            let artifacts = build_project(&project)?;
            println!("Build finished in {} ms.", start.elapsed().as_millis());

            let start = Instant::now();
            println!("Programming {} ...", artifacts.hex().display());
            program(artifacts.elf())?;
            println!(
                "Programming finished in {} ms.",
                start.elapsed().as_millis()
            );
        }
    }

    Ok(())
}
