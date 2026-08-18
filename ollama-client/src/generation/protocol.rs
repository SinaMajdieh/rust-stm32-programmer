use serde::Serialize;

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
            temperature: options.temperature(),
            seed: options.seed(),
            num_ctx: options.context_length(),
            num_predict: options.maximum_output_tokens(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct GenerateBody<'a> {
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

impl<'a> From<&'a GenerateRequest> for GenerateBody<'a> {
    fn from(request: &'a GenerateRequest) -> Self {
        Self {
            model: request.model(),
            prompt: request.prompt(),
            stream: false,
            system: request.system_prompt(),
            think: request.thinking(),
            keep_alive: request.keep_alive(),
            options: GenerateOptionsBody::from(request.options()),
        }
    }
}
