use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OllamaModel {
    #[serde(rename = "llama2")]
    Llama2,
    #[serde(rename = "llama2-uncensored")]
    Llama2Uncensored,
    #[default]
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "mixtral")]
    Mixtral,
}

impl LlmBackend for OllamaBackend {
    async fn generate(&self, prompt: &str) -> Result<String, GenerateError> {
        generate_ollama(&self.url, self.model, prompt).await
    }
}

#[async_recursion::async_recursion]
async fn generate_ollama(
    url: &str,
    model: OllamaModel,
    prompt: &str,
) -> Result<String, GenerateError> {
    let client = reqwest::Client::new();

    // Generate response from Ollama.
    let response = client
        .post(format!("{}/api/generate", url))
        .json(&OllamaGenerate {
            model,
            prompt: prompt.to_string(),
        })
        .send()
        .await
        .map_err(|e| GenerateError::BackendError(e.to_string()))?;

    let mut stream = response.bytes_stream();

    let mut text = String::new();

    while let Some(res) = stream.next().await {
        let chunk = res.map_err(|e| GenerateError::BackendError(e.to_string()))?;
        let text_chunk = String::from_utf8_lossy(&chunk);

        if let Ok(error) = serde_json::from_str::<OllamaError>(&text_chunk) {
            // If model needs to be pulled, pull it and try again.
            // Example error: "model 'mistral' not found, try pulling it first"
            if error.error.contains("try pulling it first") {
                let res = client
                    .post(format!("{}/api/pull", url))
                    .json(&OllamaPull { name: model })
                    .send()
                    .await
                    .map_err(|e| GenerateError::BackendError(e.to_string()))?;

                let mut stream = res.bytes_stream();
                let mut last_status = String::new();

                while let Some(res) = stream.next().await {
                    if let Ok(bytes) = res {
                        let text = String::from_utf8_lossy(&bytes);

                        if let Ok(status) = serde_json::from_str::<OllamaStatus>(&text) {
                            if status.status == "success" {
                                return generate_ollama(url, model, prompt).await;
                            }

                            if status.status == last_status {
                                continue;
                            }
                            info!("Ollama status: {}", status.status);
                            last_status.clone_from(&status.status);
                        }
                    }
                }
            } else {
                return Err(GenerateError::BackendError(error.error));
            }
        }

        if let Ok(response) = serde_json::from_str::<OllamaResponse>(&text_chunk) {
            text.push_str(&response.response);
        }
    }

    debug!("Ollama response: {}", text);

    Ok(text)
}

#[derive(Debug, Serialize)]
struct OllamaPull {
    name: OllamaModel,
}

#[derive(Debug, Deserialize)]
struct OllamaStatus {
    status: String,
}

#[derive(Debug, Deserialize)]
struct OllamaError {
    error: String,
}

#[derive(Debug, Serialize)]
struct OllamaGenerate {
    model: OllamaModel,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    const TEST_PROMPT: &str = "What letter comes after A?";

    #[tokio::test]
    #[traced_test]
    async fn test_ollama_backend() {
        let backend = OllamaBackend::default();

        let mut response = backend.generate(TEST_PROMPT).await.unwrap();
        response.make_ascii_lowercase();

        assert!(response.contains('b'));
    }
}
