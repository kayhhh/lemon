use serde_json::json;

use crate::{GenerateError, LlmBackend};

const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

pub struct OllamaBackend {
    pub model: OllamaModel,
    pub url: String,
}

impl Default for OllamaBackend {
    fn default() -> Self {
        Self {
            model: OllamaModel::default(),
            url: DEFAULT_OLLAMA_URL.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum OllamaModel {
    Llama2,
    Llama2Uncensored,
    #[default]
    Mistral7B,
}

impl OllamaModel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Llama2 => "llama2",
            Self::Llama2Uncensored => "llama2-uncensored",
            Self::Mistral7B => "mistral",
        }
    }
}

impl LlmBackend for OllamaBackend {
    async fn generate(&self, prompt: &str) -> Result<String, GenerateError> {
        // Pull the model if it's not already downloaded.
        reqwest::Client::new()
            .post(format!("{}/api/pull", self.url))
            .json(&json!({
                "name": self.model.as_str(),
            }))
            .send()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        // Run the model.
        let response = reqwest::Client::new()
            .post(format!("{}/api/generate", self.url))
            .json(&json!({
                "model": self.model.as_str(),
                "prompt": prompt,
            }))
            .send()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        Ok(text
            .lines()
            .map(|line| {
                // Extract the text from the JSON response.
                let json: serde_json::Value = serde_json::from_str(line).unwrap();
                json["response"].as_str().unwrap_or_default().to_string()
            })
            .collect::<String>()
            .trim()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use tracing::debug;
    use tracing_test::traced_test;

    use super::*;

    const TEST_PROMPT: &str = "What letter comes after A?";

    #[tokio::test]
    #[traced_test]
    async fn test_ollama_backend() {
        let backend = OllamaBackend::default();
        let response = backend.generate(TEST_PROMPT).await.unwrap();

        debug!("Response: {}", response);

        assert!(response.contains('B') || response.contains('b'));
    }
}
