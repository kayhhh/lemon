//! LLM nodes for [lemon-graph](https://github.com/unavi-xyz/lemon/tree/main/crates/lemon-graph).

use std::{collections::HashMap, future::Future, sync::Arc};

use lemon_graph::{
    nodes::{AsyncNode, Node},
    Data, GraphNode,
};
use thiserror::Error;
use tracing::{debug, error};

#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "replicate")]
pub mod replicate;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Backend error: {0}")]
    BackendError(String),
}

pub trait LlmBackend {
    fn generate(&self, prompt: &str) -> impl Future<Output = Result<String, GenerateError>>;
}

pub struct LlmNode<T: LlmBackend + 'static> {
    pub backend: Arc<T>,
    pub prompt: String,
}

impl<T: LlmBackend> LlmNode<T> {
    pub fn new(backend: T) -> Self {
        Self {
            backend: Arc::new(backend),
            prompt: String::new(),
        }
    }
}

impl<T: LlmBackend> Node for LlmNode<T> {
    fn process_input(&mut self, input: HashMap<String, Data>) {
        if let Some(Data::String(prompt)) = input.get("prompt") {
            self.prompt = prompt.clone();
        }
    }
}

impl<T: LlmBackend> AsyncNode for LlmNode<T> {
    fn run(&mut self) -> Box<dyn Future<Output = Option<Data>> + Unpin> {
        let backend = self.backend.clone();
        let prompt = self.prompt.clone();

        Box::new(Box::pin(async move {
            let response = backend.generate(&prompt).await;

            match response {
                Ok(output) => {
                    debug!("Generated: {}", output);
                    Some(Data::String(output))
                }
                Err(e) => {
                    error!("Error: {}", e);
                    None
                }
            }
        }))
    }
}

impl<T: LlmBackend> From<LlmNode<T>> for GraphNode {
    fn from(value: LlmNode<T>) -> Self {
        Self::Async(Box::new(value))
    }
}
