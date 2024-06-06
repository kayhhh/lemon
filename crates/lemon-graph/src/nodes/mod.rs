use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};
use std::future::Future;
use thiserror::Error;

use crate::{Graph, GraphEdge, Value};

mod core;
mod store;

pub use core::*;
pub use store::*;

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

pub trait Node: Copy + Into<NodeIndex> {
    fn input_stores(self, graph: &Graph) -> impl Iterator<Item = Store> + '_ {
        graph
            .edges_directed(self.into(), Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataMap(_)))
            .map(|edge| Store(edge.source()))
    }
    fn output_stores(self, graph: &Graph) -> impl Iterator<Item = Store> + '_ {
        graph
            .edges_directed(self.into(), Direction::Outgoing)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataMap(_)))
            .map(|edge| Store(edge.target()))
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
