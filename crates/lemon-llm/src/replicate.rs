use std::collections::HashMap;

use replicate_rust::{config::Config, Replicate};

use crate::{GenerateError, LlmBackend};

pub struct ReplicateBackend {
    pub model: ReplicateModel,
    config: Config,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ReplicateModel {
    Llama2,
    #[default]
    Mistral7B,
}

impl ReplicateModel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Llama2 => "meta/llama-2-7b:73001d654114dad81ec65da3b834e2f691af1e1526453189b7bf36fb3f32d0f9",
            Self::Mistral7B => "mistralai/mistral-7b-instruct-v0.1:83b6a56e7c828e667f21fd596c338fd4f0039b46bcfa18d973e8e70e455fda70",
        }
    }
}

impl ReplicateBackend {
    pub fn new(model: ReplicateModel, config: Config) -> Self {
        Self { model, config }
    }
}

impl LlmBackend for ReplicateBackend {
    async fn generate(&self, prompt: &str) -> Result<String, GenerateError> {
        let replicate = Replicate::new(self.config.clone());

        let mut inputs = HashMap::new();
        inputs.insert("prompt", prompt);

        let result = replicate
            .run(self.model.as_str(), inputs)
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        let output = result
            .output
            .ok_or(GenerateError::BackendError("No output".to_string()))?;

        let array = output.as_array().ok_or(GenerateError::BackendError(
            "Output is not an array".to_string(),
        ))?;

        Ok(array
            .iter()
            .map(|x| x.as_str().unwrap_or_default())
            .collect::<String>()
            .trim()
            .to_string())
    }
}
