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

    use lemon_graph::{nodes::AsyncNode, ExecutionStep, Graph, GraphNode, Value};
    use petgraph::Direction;

    use crate::{LlmNode, LlmWeight};

    use super::*;

    const TEST_PROMPT: &str = "What letter comes after A?";

    #[tokio::test]
    async fn test_backend() {
        let backend = OllamaBackend::default();
        let response = backend.generate(TEST_PROMPT).await.unwrap();

        assert!(!response.is_empty());
    }

    #[tokio::test]
    async fn test_weight() {
        let backend = OllamaBackend::default();
        let weight = LlmWeight {
            backend: Arc::new(backend),
        };
        let response = weight
            .run(vec![TEST_PROMPT.to_string().into()])
            .await
            .unwrap();

        assert!(!response.is_empty());
    }

    #[tokio::test]
    async fn test_node() {
        let backend = OllamaBackend::default();

        let weight = LlmWeight {
            backend: Arc::new(backend),
        };

        let mut graph = Graph::new();
        let node = LlmNode::new(&mut graph, weight);

        node.set_prompt(&mut graph, TEST_PROMPT.to_string())
            .unwrap();

        let step = ExecutionStep(node.0);
        let _ = step.execute(&mut graph).await.unwrap();

        let output = graph
            .neighbors_directed(node.0, Direction::Outgoing)
            .next()
            .unwrap();
        let output = graph.node_weight(output).unwrap();
        let output = match output {
            GraphNode::Store(Value::String(output)) => output,
            _ => panic!(),
        };

        assert!(!output.is_empty());
    }
}
