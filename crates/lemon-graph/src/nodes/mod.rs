use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};
use std::future::Future;
use thiserror::Error;

mod callback;
mod log;
mod prompt;

pub use callback::CallbackNode;
pub use log::LogNode;
pub use prompt::PromptNode;

use crate::{Graph, GraphEdge, GraphNode, Value};

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Missing input at index {0}")]
    MissingInput(usize),
    #[error("Conversion error, got {0:?}")]
    ConversionError(Value),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait AsyncNode {
    fn run(
        &self,
        inputs: Vec<Value>,
    ) -> Box<dyn Future<Output = Result<Vec<Value>, NodeError>> + Unpin>;
}

pub trait SyncNode {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError>;
}

pub trait NodeWrapper: Copy + Into<NodeIndex> {
    fn input_stores(self, graph: &Graph) -> impl Iterator<Item = StoreWrapper> + '_ {
        graph
            .edges_directed(self.into(), Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataMap(_)))
            .map(|edge| StoreWrapper(edge.source()))
    }
    fn output_stores(self, graph: &Graph) -> impl Iterator<Item = StoreWrapper> + '_ {
        graph
            .edges_directed(self.into(), Direction::Outgoing)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataMap(_)))
            .map(|edge| StoreWrapper(edge.target()))
    }

    fn input_execution(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        graph
            .edges_directed(self.into(), Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::ExecutionFlow))
            .map(|edge| edge.source())
    }
    fn output_execution(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        graph
            .edges_directed(self.into(), Direction::Outgoing)
            .filter(|edge| matches!(edge.weight(), GraphEdge::ExecutionFlow))
            .map(|edge| edge.target())
    }

    /// Adds an execution flow from the given node to this node.
    fn run_after(self, graph: &mut Graph, node: NodeIndex) {
        graph.add_edge(node, self.into(), GraphEdge::ExecutionFlow);
    }

    /// Adds an execution flow from this node to the given node.
    fn run_before(self, graph: &mut Graph, node: NodeIndex) {
        graph.add_edge(self.into(), node, GraphEdge::ExecutionFlow);
    }
}

#[derive(Debug, Error)]
pub enum GetStoreError {
    #[error("No store found")]
    NoStore,
}

#[derive(Debug, Clone, Copy)]
pub struct StoreWrapper(pub NodeIndex);

impl StoreWrapper {
    /// Returns an iterator over any input stores.
    pub fn inputs(self, graph: &Graph) -> impl Iterator<Item = StoreWrapper> + '_ {
        graph
            .edges_directed(self.0, Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataFlow))
            .map(|edge| StoreWrapper(edge.source()))
    }
    /// Returns an iterator over any output stores.
    pub fn outputs(self, graph: &Graph) -> impl Iterator<Item = StoreWrapper> + '_ {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataFlow))
            .map(|edge| StoreWrapper(edge.target()))
    }

    /// Sets the input of the store.
    /// This will remove any existing inputs.
    pub fn set_input(&self, graph: &mut Graph, store: Option<StoreWrapper>) {
        // Remove any existing inputs.
        let edges = graph
            .edges_directed(self.0, Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataFlow))
            .map(|edge| edge.id())
            .collect::<Vec<_>>();

        for edge in edges {
            graph.remove_edge(edge);
        }

        // Set the new input.
        if let Some(store) = store {
            graph.add_edge(store.0, self.0, GraphEdge::DataFlow);
        }
    }

    /// Adds an output edge to the given store.
    pub fn add_output(&self, graph: &mut Graph, store: StoreWrapper) {
        graph.add_edge(self.0, store.0, GraphEdge::DataFlow);
    }

    /// Sets the default value of the store.
    /// This will be used if no input is set.
    pub fn set_value(&self, graph: &mut Graph, value: Value) {
        graph[self.0] = GraphNode::Store(value);
    }
}
