use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};
use thiserror::Error;

use crate::{Graph, GraphEdge, GraphNode, Value};

#[derive(Debug, Error)]
pub enum GetStoreError {
    #[error("No store found")]
    NoStore,
}

/// Stores data, for transfer between nodes.
#[derive(Debug, Clone, Copy)]
pub struct Store(pub NodeIndex);

impl Store {
    /// Returns an iterator over any input stores.
    pub fn inputs(self, graph: &Graph) -> impl Iterator<Item = Store> + '_ {
        graph
            .edges_directed(self.0, Direction::Incoming)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataFlow))
            .map(|edge| Store(edge.source()))
    }
    /// Returns an iterator over any output stores.
    pub fn outputs(self, graph: &Graph) -> impl Iterator<Item = Store> + '_ {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter(|edge| matches!(edge.weight(), GraphEdge::DataFlow))
            .map(|edge| Store(edge.target()))
    }

    /// Sets the input of the store.
    /// This will remove any existing inputs.
    pub fn set_input(&self, graph: &mut Graph, store: Option<Store>) {
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
    pub fn add_output(&self, graph: &mut Graph, store: Store) {
        graph.add_edge(self.0, store.0, GraphEdge::DataFlow);
    }

    /// Sets the default value of the store.
    /// This will be used if no input is set.
    pub fn set_value(&self, graph: &mut Graph, value: Value) {
        graph[self.0] = GraphNode::Store(value);
    }
}
