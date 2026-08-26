# Ollama Client

A Rust client library for interacting with the [Ollama](https://ollama.com/) API.

The library provides a typed interface for communicating with an Ollama server, generating text using locally available models, retrieving information about installed and loaded models, and querying server and model metadata.

## Overview

This project implements a Rust client for the Ollama HTTP API.

The main goal of the project is to provide a convenient and type-safe interface for interacting with Ollama from Rust applications. Instead of constructing HTTP requests and manually handling JSON data in application code, the client exposes Rust types and methods representing the operations supported by the library.

The client currently supports:

- Connecting to an Ollama server.
- Retrieving the Ollama server version.
- Listing models installed on the server.
- Listing models currently loaded by the server.
- Retrieving detailed metadata about a model.
- Generating text using an Ollama model.
- Configuring generation parameters such as temperature, seed, context length, and maximum output tokens.
- Providing system prompts and other generation controls.
- Reporting generation statistics such as token counts, execution durations, and tokens per second.
- Handling URL, HTTP, timeout, API, and deserialization errors.

## Project Structure

The project is organized into several modules based on their responsibilities:

```text
src/
├── client.rs
├── error.rs
├── generation/
│   ├── mod.rs
│   ├── wire.rs
│   └── tests.rs
├── model/
│   ├── mod.rs
│   ├── wire.rs
│   └── tests.rs
├── version.rs
└── lib.rs

examples/
└── main.rs

tests/
└── live_ollama.rs
```

### `client`

The `client` module contains the main `OllamaClient` type. It is responsible for maintaining the HTTP client and the base URL of the Ollama server.

It also provides the common functionality used by the other modules for constructing GET and POST requests and executing requests that return JSON responses.

### `generation`

The `generation` module contains the public types used for text generation, including:

- `GenerateRequest`
- `GenerateOptions`
- `Generation`

The module also contains the internal wire types used to translate between the public Rust API and the JSON format expected by Ollama.

### `model`

The `model` module provides types and methods for working with Ollama models. It supports installed models, loaded models, and detailed model metadata.

### `version`

The `version` module provides access to the version reported by the Ollama server.

### `error`

The `error` module defines the error type returned by the client and provides a common `Result<T>` type.

### Tests

The project contains both unit tests and integration tests.

The unit tests verify serialization and deserialization without requiring a running Ollama server. The integration tests communicate with an actual local Ollama instance and are therefore marked as ignored by default.

## Client Architecture

The main entry point of the library is `OllamaClient`.

A client is created by providing the URL of an Ollama server:

```rust
use ollama_client::OllamaClient;

let client = OllamaClient::new("http://localhost:11434")?;
```

The client stores a reusable `reqwest::Client` together with the server's base URL.

Operations such as model retrieval and text generation are implemented as methods on `OllamaClient`.


## API Communication

The client communicates with Ollama through its HTTP API.

The implemented operations correspond to the following endpoints:

|Operation|HTTP method|Endpoint|
|---|---|---|
|Server version|GET|`/api/version`|
|List installed models|GET|`/api/tags`|
|List loaded models|GET|`/api/ps`|
|Model metadata|POST|`/api/show`|
|Generate response|POST|`/api/generate`|

Requests are sent asynchronously using `reqwest`.

The client applies a timeout to every request. Successful responses are deserialized directly into the corresponding Rust response type.

If Ollama returns an unsuccessful HTTP status, the client attempts to extract the API error message from the response body and returns it through the library's error type.

## Text Generation

The `generate` method performs a non-streaming generation request:

```rust
let generation = client
    .generate(&request, Duration::from_secs(120))
    .await?;
```

The result is represented by the `Generation` type.

It contains both the generated response and information reported by Ollama about the generation process, including:

- Generated text.
- Thinking output, when provided.
- Completion status.
- Completion reason.
- Total processing duration.
- Model loading duration.
- Number of prompt tokens.
- Number of generated tokens.
- Prompt evaluation duration.
- Generation evaluation duration.

## Working With Models

The client provides several operations for inspecting models on the Ollama server.

### Installed Models

Installed models can be retrieved using:

```rust
let models = client
    .list_models(Duration::from_secs(10))
    .await?;
```

### Loaded Models

Models currently loaded by the Ollama server can be retrieved using:

```rust
let models = client
    .loaded_models(Duration::from_secs(10))
    .await?;
```

In addition to basic model information, `LoadedModel` contains information such as VRAM usage, context length, and expiration time.

### Model Metadata

Detailed metadata for a particular model can be retrieved with:

```rust
let metadata = client
    .model_metadata("qwen2.5-coder:7b", Duration::from_secs(10))
    .await?;
```

The metadata type also preserves additional model information returned by Ollama in a `serde_json::Map`.

This allows the client to expose known model properties through typed fields while still retaining information that may vary between models or Ollama versions.

## Error Handling

The library uses a single `Error` enum for errors that can occur while interacting with Ollama.

The current error categories are:

```text
Error
├── Url
├── Http
├── Timeout
└── Api
```

### URL Errors

An invalid server URL is reported as an `Error::Url`.

### HTTP Errors

Transport-level failures are represented by `Error::Http`.

### Timeout Errors

Timeouts are represented separately by `Error::Timeout`. The configured timeout duration is preserved in the error, allowing the caller to determine that the operation failed specifically because it exceeded the requested time limit.

### API Errors

When Ollama returns an unsuccessful HTTP status, the client creates an `Error::Api` containing the HTTP status code and the error message returned by Ollama.

The error type also provides helper methods such as:

```rust
error.is_timeout()
```

and:

```rust
error.status()
```

This allows applications to inspect the type of failure without having to match against every possible error variant.

## Example

A complete example is included in `examples/main.rs`.

The example creates an `OllamaClient`, configures generation parameters, sends a prompt to an Ollama model, and prints the generated response and generation performance.

It demonstrates the intended high-level usage of the library without requiring the application to manually construct HTTP requests or JSON payloads.

The example can be run with:

```bash
cargo run --example main
```

This requires an Ollama server to be running locally and the model specified in the example to be installed.

## Limitations

The current implementation intentionally focuses on a subset of the Ollama API.
In particular, text generation is currently implemented as a non-streaming operation. The client waits for the complete generation response before returning it to the caller.
Additional Ollama API functionality could be added in the future, such as streaming generation and other model-management operations.
