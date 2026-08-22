use serde::{Deserialize, Serialize};

use super::{InstalledModel, LoadedModel};

#[derive(Debug, Serialize)]
pub(super) struct ShowModelRequest<'a> {
    model: &'a str,
    verbose: bool,
}

impl<'a> ShowModelRequest<'a> {
    pub(super) fn new(model: &'a str) -> Self {
        Self {
            model,
            verbose: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct ListModelsResponse {
    pub(super) models: Vec<InstalledModel>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ListLoadedModelsResponse {
    pub(super) models: Vec<LoadedModel>,
}
