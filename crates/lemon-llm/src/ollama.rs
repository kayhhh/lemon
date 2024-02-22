use serde_json::json;

use crate::{GenerateError, LlmBackend};

const OLLAMA_URL: &str = "http://localhost:11434";

pub struct OllamaBackend {
    pub model: OllamaModel,
    pub url: String,
}

impl Default for OllamaBackend {
    fn default() -> Self {
        Self {
            model: OllamaModel::default(),
            url: OLLAMA_URL.to_string(),
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
            .post(format!("{}/api/pull", OLLAMA_URL))
            .json(&json!({
                "name": self.model.as_str(),
            }))
            .send()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        // Run the model.
        let response = reqwest::Client::new()
            .post(format!("{}/api/generate", OLLAMA_URL))
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
    use std::sync::Arc;

    use lemon_graph::{Data, Engine, GraphEdge, GraphNode};

    use crate::LlmNode;

    use super::*;

    #[tokio::test]
    async fn test_generate() {
        let backend = OllamaBackend::default();
        let response = backend.generate("Tell me a short joke.").await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_node() {
        let backend = OllamaBackend::default();

        let node = LlmNode {
            backend: Arc::new(backend),
            prompt: "Tell me a short joke.".to_string(),
        };

        let mut engine = Engine::default();

        let llm = engine.0.add_node(GraphNode::Async(Box::new(node)));
        let trigger = engine.0.add_node(GraphNode::Trigger("start".to_string()));
        engine.0.add_edge(trigger, llm, GraphEdge::Flow);

        let result = engine
            .execute("start")
            .await
            .expect("Failed to execute graph");

        assert!(matches!(result, Data::String(_)));
    }
}
