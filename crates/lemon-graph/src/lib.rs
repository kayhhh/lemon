//! Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).
//!
//! # Usage
//!
//! ```
//! use lemon_graph::Graph;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut graph = Graph::new();
//!
//!     // TODO
//! }
//! ```

use nodes::{AsyncNode, SyncNode};
use petgraph::graph::DiGraph;

pub mod execution;
pub mod nodes;
mod value;

pub use value::Value;

#[derive(Debug, Clone, Copy)]
pub enum GraphEdge {
    /// Execution flow between nodes.
    ExecutionFlow,
    /// Data flow between stores.
    DataFlow,
    /// Data map from node -> store, or store -> node.
    /// The usize is the index of the data in the node.
    DataMap(usize),
}

pub enum GraphNode {
    /// Executable async node.
    AsyncNode(Box<dyn AsyncNode>),
    /// Executable sync node.
    SyncNode(Box<dyn SyncNode>),
    /// Used as an intermediate store for data between nodes.
    Store(Value),
}

pub type Graph = DiGraph<GraphNode, GraphEdge>;
