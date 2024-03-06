//! LLM nodes for [lemon-graph](https://github.com/unavi-xyz/lemon/tree/main/crates/lemon-graph).
//!
//! ## Usage
//!
//! ```
//! use std::sync::Arc;
//!
//! use lemon_graph::{Graph, Executor, nodes::{NodeWrapper, LogNode}};
//! use lemon_llm::{ollama::{OllamaBackend, OllamaModel}, LlmBackend, LlmNode, LlmWeight};
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut graph = Graph::default();
//!
//!    // Create a new Ollama backend.
//!    let backend = Arc::new(OllamaBackend {
//!        model: OllamaModel::Mistral7B,
//!        ..Default::default()
//!    });
//!
//!    // Create an llm node, using our Ollama backend.
//!    let llm = LlmNode::new(&mut graph, LlmWeight::new(backend.clone()));
//!
//!    // Set the prompt manually.
//!    let prompt = llm.prompt(&graph).unwrap();
//!    prompt.set_value(
//!         &mut graph,
//!         "Tell me your favorite lemon fact.".to_string().into(),
//!    );
//!
//!    // Connect the reponse to a log node.
//!    let response = llm.response(&graph).unwrap();
//!
//!    let log = LogNode::new(&mut graph);
//!    log.run_after(&mut graph, llm.0);
//!
//!    let message = log.message(&graph).unwrap();
//!    message.set_input(&mut graph, Some(response));
//!
//!    // Execute the graph.
//!    // Executor::execute(&mut graph, llm.0).await.unwrap();
//! }
//! ```

use std::{future::Future, sync::Arc};

use lemon_graph::{
    nodes::{AsyncNode, GetStoreError, NodeError, NodeWrapper, StoreWrapper},
    Graph, GraphEdge, GraphNode, Value,
};
use petgraph::graph::NodeIndex;
use thiserror::Error;
use tracing::error;

#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "replicate")]
pub mod replicate;

#[derive(Debug, Clone, Copy)]
pub struct LlmNode(pub NodeIndex);

impl From<LlmNode> for NodeIndex {
    fn from(value: LlmNode) -> Self {
        value.0
    }
}

impl NodeWrapper for LlmNode {}

impl LlmNode {
    pub fn new<T: LlmBackend>(graph: &mut Graph, weight: LlmWeight<T>) -> Self {
        let index = graph.add_node(GraphNode::AsyncNode(Box::new(weight)));

        let input = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, index, GraphEdge::DataMap(0));

        let output = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(index, output, GraphEdge::DataMap(0));

        Self(index)
    }

    pub fn prompt(&self, graph: &Graph) -> Result<StoreWrapper, GetStoreError> {
        self.input_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }

    pub fn response(&self, graph: &Graph) -> Result<StoreWrapper, GetStoreError> {
        self.output_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }
}

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Backend error: {0}")]
    BackendError(String),
}

pub trait LlmBackend {
    fn generate(&self, prompt: &str) -> impl Future<Output = Result<String, GenerateError>>;
}

pub struct LlmWeight<T: LlmBackend + 'static> {
    pub backend: Arc<T>,
}

impl<T: LlmBackend> LlmWeight<T> {
    pub fn new(backend: Arc<T>) -> Self {
        Self { backend }
    }
}

impl<T: LlmBackend> AsyncNode for LlmWeight<T> {
    fn run(
        &self,
        inputs: Vec<Value>,
    ) -> Box<dyn Future<Output = Result<Vec<Value>, NodeError>> + Unpin> {
        let backend = self.backend.clone();

        Box::new(Box::pin(async move {
            let prompt = match inputs.first() {
                Some(Value::String(prompt)) => prompt.clone(),
                Some(v) => return Err(NodeError::ConversionError(v.clone())),
                None => return Err(NodeError::MissingInput(0)),
            };

            let response = backend
                .generate(&prompt)
                .await
                .map_err(|e| NodeError::InternalError(format!("Failed to generate: {}", e)))?;

            Ok(vec![Value::String(response)])
        }))
    }
}
