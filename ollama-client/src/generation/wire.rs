use serde::{Deserialize, Serialize};

use super::{GenerateOptions, GenerateRequest};

#[derive(Debug, Serialize)]
struct GenerateOptionsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_ctx: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

impl GenerateOptionsBody {
    fn is_empty(&self) -> bool {
        self.temperature.is_none()
            && self.seed.is_none()
            && self.num_ctx.is_none()
            && self.num_predict.is_none()
    }
}

impl From<&GenerateOptions> for GenerateOptionsBody {
    fn from(options: &GenerateOptions) -> Self {
        Self {
            temperature: options.temperature,
            seed: options.seed,
            num_ctx: options.context_length,
            num_predict: options.maximum_output_tokens,
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct GenerateRequestBody<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    think: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keep_alive: Option<&'a str>,

    #[serde(skip_serializing_if = "GenerateOptionsBody::is_empty")]
    options: GenerateOptionsBody,
}

impl<'a> From<&'a GenerateRequest> for GenerateRequestBody<'a> {
    fn from(request: &'a GenerateRequest) -> Self {
        Self {
            model: &request.model,
            prompt: &request.prompt,
            stream: false,
            system: request.system_prompt.as_deref(),
            think: request.thinking,
            keep_alive: request.keep_alive.as_deref(),
            options: GenerateOptionsBody::from(&request.options),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct GenerateResponseBody {
    pub(super) response: String,

    #[serde(default)]
    pub(super) thinking: String,

    pub(super) done: bool,
    pub(super) done_reason: Option<String>,
    pub(super) total_duration: u64,
    pub(super) load_duration: u64,
    pub(super) prompt_eval_count: u64,
    pub(super) prompt_eval_duration: u64,
    pub(super) eval_count: u64,
    pub(super) eval_duration: u64,
}
