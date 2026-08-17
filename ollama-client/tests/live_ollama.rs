use ollama_client::{GenerateRequest, OllamaClient};

#[tokio::test]
#[ignore = "requires a running local Ollama Server"]
async fn retrieves_ollama_version() {
    let client = OllamaClient::new("http://localhost:11434").expect("Local Url sould be valid");

    let version = client
        .version()
        .await
        .expect("Ollama should return its version");

    assert!(!version.as_str().trim().is_empty());

    println!("Ollama Version: {}", version.as_str())
}

#[tokio::test]
#[ignore = "requires a running Ollama server and OLLAMA_TEST_MODEL"]
async fn generates_a_response() {
    let model =
        std::env::var("OLLAMA_TEST_MODEL").expect("OLLAMA_TEST_MODEL must name an installed model");

    let client = OllamaClient::new("http://localhost:11434").expect("local URL should be valid");

    let request = GenerateRequest::new(model, "Respond with exactly the word READY.");

    let generation = client
        .generate(&request)
        .await
        .expect("generation should succeed");

    assert!(generation.is_done());
    assert!(!generation.response().trim().is_empty());
    assert!(generation.generated_tokens() > 0);

    println!("Response: {}", generation.response());
    println!("Generated tokens: {}", generation.generated_tokens());
    println!(
        "Generation speed: {:?} tokens/s",
        generation.tokens_per_second()
    );
}
