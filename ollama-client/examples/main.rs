use std::time::Duration;

use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};

const OLLAMA_URL: &str = "http://localhost:11434";
const MODEL: &str = "qwen2.5-coder:7b";

#[tokio::main]
async fn main() {
    // Ollama client to use the APIs.
    let a = OllamaClient::new(OLLAMA_URL);
    let Ok(client) = a else {
        eprintln!("Client Initializiation failes: {:?}", a.unwrap_err());
        return;
    };

    // Ollama Options.
    let options = GenerateOptions::new()
        // .with_seed(329)
        .with_temperature(0.05)
        .with_context_length(8192)
        .with_maximum_output_tokens(1024);

    // Request
    let request = GenerateRequest::new(MODEL, "Make the onboard LED blink every 500 ms.")
        .with_system_prompt(SYSTEM_PROMPT)
        .with_thinking(false)
        .with_keep_alive("2m")
        .with_options(options);

    // Generation
    match client.generate(&request, Duration::from_secs(120)).await {
        Ok(generation) => {
            println!("Responce: {}", generation.response);
            println!("Generated tokens: {}", generation.generated_tokens);
            println!(
                "Generation Speed: {} tokens/s",
                generation.tokens_per_second().unwrap_or(f64::NAN),
            );
        }
        Err(error) => {
            eprintln!("Generation failed: {:#?}", error);
        }
    }
}

const SYSTEM_PROMPT: &str = r#"
You are an expert embedded systems engineer specializing in bare-metal C programming for ARM Cortex-M microcontrollers, with deep knowledge of the STM32F103 family.

Your task is to generate correct, minimal, readable, educational bare-metal C code for the user's STM32 project.

TARGET PLATFORM

- MCU: STM32F103C8T6
- Core: ARM Cortex-M3
- Board: common STM32F103C8T6 "Blue Pill"
- Architecture: ARM Thumb
- Language: C
- Programming style: true bare-metal register-level programming

PROJECT ENVIRONMENT

This is a minimal bare-metal STM32 project. Do not use any external libraries.


"#;
