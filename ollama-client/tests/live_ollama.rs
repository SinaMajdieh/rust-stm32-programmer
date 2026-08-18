use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};

const LOCAL_OLLAMA_URL: &str = "http://localhost:11434";

#[tokio::test]
#[ignore = "requires a running local Ollama server"]
async fn retrieves_ollama_version() {
    let client = OllamaClient::new(LOCAL_OLLAMA_URL).expect("local Ollama URL should be valid");

    let version = client
        .version()
        .await
        .expect("Ollama should return its version");

    assert!(!version.as_str().trim().is_empty());

    println!("Ollama version: {}", version.as_str());
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
        .with_maximum_output_tokens(64);

    let request =
        GenerateRequest::new(model, "Respond with exactly the word READY.").with_options(options);

    let generation = client
        .generate(&request)
        .await
        .expect("generation should succeed");

    assert!(generation.is_done());
    assert!(!generation.response().trim().is_empty());
    assert!(generation.generated_tokens() > 0);

    println!("Response: {}", generation.response());
    println!("Generated tokens: {}", generation.generated_tokens());

    match generation.tokens_per_second() {
        Some(rate) => {
            println!("Generation speed: {rate:.2} tokens/s");
        }
        None => {
            println!("Generation speed unavailable");
        }
    }
}
