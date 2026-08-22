use serde_json::Value;

use super::{
    ModelMetadata,
    wire::{ListLoadedModelsResponse, ListModelsResponse, ShowModelRequest},
};

#[test]
fn deserializes_installed_models() {
    let json = r#"
    {
        "models": [
            {
                "name": "qwen2.5-coder:7b-instruct",
                "model": "qwen2.5-coder:7b-instruct",
                "modified_at": "2026-08-19T12:00:00Z",
                "size": 4683075271,
                "digest": "abc123",
                "details": {
                    "format": "gguf",
                    "family": "qwen2",
                    "families": ["qwen2"],
                    "parameter_size": "7.6B",
                    "quantization_level": "Q4_K_M"
                }
            }
        ]
    }
    "#;

    let response: ListModelsResponse =
        serde_json::from_str(json).expect("installed model response should deserialize");

    let model = &response.models[0];

    assert_eq!(model.name, "qwen2.5-coder:7b-instruct");
    assert_eq!(model.size_bytes, 4_683_075_271);
    assert_eq!(model.digest, "abc123");
    assert_eq!(model.details.format, "gguf");
    assert_eq!(model.details.family, "qwen2");
    assert_eq!(model.details.parameter_size, "7.6B");
    assert_eq!(model.details.quantization_level, "Q4_K_M");
}

#[test]
fn deserializes_loaded_models() {
    let json = r#"
    {
        "models": [
            {
                "name": "qwen2.5-coder:7b-instruct",
                "model": "qwen2.5-coder:7b-instruct",
                "size": 4683075271,
                "digest": "abc123",
                "details": {
                    "parent_model": "",
                    "format": "gguf",
                    "family": "qwen2",
                    "families": ["qwen2"],
                    "parameter_size": "7.6B",
                    "quantization_level": "Q4_K_M"
                },
                "expires_at": "2026-08-19T12:05:00Z",
                "size_vram": 4683075271,
                "context_length": 4096
            }
        ]
    }
    "#;

    let response: ListLoadedModelsResponse =
        serde_json::from_str(json).expect("loaded model response should deserialize");

    let model = &response.models[0];

    assert_eq!(model.name, "qwen2.5-coder:7b-instruct");
    assert_eq!(model.vram_size_bytes, 4_683_075_271);
    assert_eq!(model.context_length, 4096);
    assert_eq!(model.details.family, "qwen2");
}

#[test]
fn serializes_show_model_request() {
    let body = ShowModelRequest::new("qwen2.5-coder:7b-instruct");

    let json = serde_json::to_value(body).expect("show model request should serialize");

    assert_eq!(json["model"], "qwen2.5-coder:7b-instruct");
    assert_eq!(json["verbose"], false);
}

#[test]
fn deserializes_model_metadata() {
    let json = r#"
    {
        "parameters": "temperature 0.7\nnum_ctx 32768",
        "license": "Example license",
        "modified_at": "2026-08-19T12:00:00Z",
        "template": "{{ .Prompt }}",
        "capabilities": ["completion", "tools"],
        "details": {
            "parent_model": "",
            "format": "gguf",
            "family": "qwen2",
            "families": ["qwen2"],
            "parameter_size": "7.6B",
            "quantization_level": "Q4_K_M"
        },
        "model_info": {
            "general.architecture": "qwen2",
            "qwen2.context_length": 32768,
            "qwen2.embedding_length": 3584
        }
    }
    "#;

    let metadata: ModelMetadata =
        serde_json::from_str(json).expect("model metadata should deserialize");

    assert_eq!(metadata.details.family, "qwen2");
    assert_eq!(metadata.details.parameter_size, "7.6B");
    assert_eq!(metadata.capabilities, ["completion", "tools"]);

    assert_eq!(
        metadata
            .raw_model_info
            .get("general.architecture")
            .and_then(Value::as_str),
        Some("qwen2"),
    );

    assert_eq!(
        metadata
            .raw_model_info
            .get("qwen2.context_length")
            .and_then(Value::as_u64),
        Some(32_768),
    );
}
