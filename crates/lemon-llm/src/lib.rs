use std::{collections::HashMap, future::Future};

use lemon_graph::{
    nodes::{AsyncNode, Node},
    Data,
};
use thiserror::Error;

#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "replicate")]
pub mod replicate;

#[derive(Debug, Error)]
pub enum GenerateError {}

pub trait LlmBackend {
    fn generate(&self, prompt: &str) -> Box<dyn Future<Output = ()> + Unpin>;
}

pub struct LlmNode<T: LlmBackend> {
    pub backend: T,
    pub prompt: String,
}

impl<T: LlmBackend> Node for LlmNode<T> {
    fn process_input(&mut self, input: HashMap<String, Data>) {
        if let Some(Data::String(prompt)) = input.get("prompt") {
            self.prompt = prompt.clone();
        }
    }
}

impl<T: LlmBackend> AsyncNode for LlmNode<T> {
    fn run(&self) -> Box<dyn Future<Output = ()> + Unpin> {
        self.backend.generate(&self.prompt)
    }
}
