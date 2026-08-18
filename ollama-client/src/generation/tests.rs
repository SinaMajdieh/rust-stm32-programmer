use super::*;

#[test]
fn omits_options_when_none_are_configured() {
    let request = GenerateRequest::new("test-model", "Hello");

    let body = GenerateBody::from(&request);

    let json = serde_json::to_value(body).expect("request body should serialize");

    assert_eq!(json["model"], "test-model");
    assert_eq!(json["prompt"], "Hello");
    assert_eq!(json["stream"], false);
    assert!(json.get("options").is_none());

    assert!(json.get("system").is_none());
    assert!(json.get("think").is_none());
    assert!(json.get("keep_alive").is_none());
    assert!(json.get("options").is_none());
}

#[test]
fn serializes_options_using_ollama_field_names() {
    let options = GenerateOptions::new()
        .with_temperature(0.0)
        .with_seed(42)
        .with_context_length(4096)
        .with_maximum_output_tokens(512);

    let request = GenerateRequest::new("test-model", "Hello").with_options(options);

    let body = GenerateBody::from(&request);

    let json = serde_json::to_value(body).expect("request body should serialize");

    let options = &json["options"];

    assert_eq!(options["temperature"], 0.0);
    assert_eq!(options["seed"], 42);
    assert_eq!(options["num_ctx"], 4096);
    assert_eq!(options["num_predict"], 512);

    assert!(options.get("context_length").is_none());
    assert!(options.get("maximum_output_tokens").is_none());
}

#[test]
fn serializes_request_controls_using_ollama_names() {
    let request = GenerateRequest::new("test-model", "Hello")
        .with_system_prompt("Return only valid C code.")
        .with_thinking(false)
        .with_keep_alive("5m");

    let body = GenerateBody::from(&request);

    let json = serde_json::to_value(body).expect("request body should serialize");

    assert_eq!(json["system"], "Return only valid C code.");
    assert_eq!(json["think"], false);
    assert_eq!(json["keep_alive"], "5m");

    assert!(json.get("system_prompt").is_none());
    assert!(json.get("thinking").is_none());
}
