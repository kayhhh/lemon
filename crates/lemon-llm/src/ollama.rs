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
            Self::Mistral7B => "mistral7b",
        }
    }
}

impl LlmBackend for OllamaBackend {
    async fn generate(&self, prompt: &str) -> Result<String, GenerateError> {
        let response = reqwest::Client::new()
            .post(format!("{}/api/generate", OLLAMA_URL))
            .json(&json!({
                "model": self.model.as_str(),
                "prompt": prompt,
            }))
            .send()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))?;

        response
            .text()
            .await
            .map_err(|e| GenerateError::BackendError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use lemon_graph::{Engine, GraphEdge, GraphNode};

    use crate::LlmNode;

    use super::*;

    #[tokio::test]
    async fn test_generate() {
        let backend = OllamaBackend::default();

        let node = LlmNode {
            backend: Arc::new(backend),
            prompt: "Once upon a time".to_string(),
        };

        let mut engine = Engine::default();

        let llm = engine.0.add_node(GraphNode::Async(Box::new(node)));
        let trigger = engine.0.add_node(GraphNode::Trigger("start".to_string()));
        engine.0.add_edge(trigger, llm, GraphEdge::Flow);

        let result = engine.execute("start").await;

        assert!(result.is_some());
    }
}
