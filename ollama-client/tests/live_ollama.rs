use std::time::Duration;

use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};
use tokio::time::Instant;

const LOCAL_OLLAMA_URL: &str = "http://localhost:11434";

#[tokio::test]
#[ignore = "requires a running local Ollama server"]
async fn retrieves_ollama_version() {
    let client = OllamaClient::new(LOCAL_OLLAMA_URL).expect("local Ollama URL should be valid");

    let version = client
        .version(Duration::from_secs(10))
        .await
        .expect("Ollama should return its version");

    assert!(!version.as_str().trim().is_empty());

    println!("Ollama version: {version}");
}

#[tokio::test]
#[ignore = "requires Ollama and OLLAMA_TEST_MODEL"]
async fn generates_a_response() {
    let model =
        std::env::var("OLLAMA_TEST_MODEL").expect("OLLAMA_TEST_MODEL must name an installed model");

    let client = OllamaClient::new(LOCAL_OLLAMA_URL).expect("local Ollama URL should be valid");

    let options = GenerateOptions::new()
        .with_temperature(0.0)
        .with_seed(42)
        .with_context_length(4096)
        .with_maximum_output_tokens(256);

    let request = GenerateRequest::new(
        model,
        "Write a C function that returns the larger \
         of two integers.",
    )
    .with_system_prompt("Comment all the steps and only return C code.")
    .with_thinking(false)
    .with_keep_alive("5m")
    .with_options(options);

    let start = Instant::now();
    let generation = client
        .generate(&request, Duration::from_secs(120))
        .await
        .expect("generation should succeed");
    let wall_time = start.elapsed();

    assert!(generation.done);
    assert!(!generation.response.trim().is_empty());
    assert!(generation.generated_tokens > 0);

    println!("Response: {}", generation.response);
    println!("Generated tokens: {}", generation.generated_tokens);

    match generation.tokens_per_second() {
        Some(rate) => {
            println!("Generation speed: {rate:.2} tokens/s");
        }
        None => {
            println!("Generation speed unavailable");
        }
    }

    eprintln!("Wall time : {:.2?}", wall_time)
}

#[tokio::test]
#[ignore = "requires Ollama and OLLAMA_TEST_MODEL"]
async fn retrieves_model_metadata() {
    let model =
        std::env::var("OLLAMA_TEST_MODEL").expect("OLLAMA_TEST_MODEL must name an installed model");

    let client = OllamaClient::new(LOCAL_OLLAMA_URL).expect("local Ollama URL should be valid");

    let metadata = client
        .model_metadata(&model, Duration::from_secs(10))
        .await
        .expect("model metadata should be available");

    assert!(!metadata.details.family.trim().is_empty());

    assert!(!metadata.details.parameter_size.trim().is_empty());

    assert!(!metadata.details.quantization_level.trim().is_empty());

    println!("Model: {model}");
    println!("Family: {}", metadata.details.family);

    println!("Parameters: {}", metadata.details.parameter_size);

    println!("Quantization: {}", metadata.details.quantization_level);

    println!("Capabilities: {:?}", metadata.capabilities);
}
