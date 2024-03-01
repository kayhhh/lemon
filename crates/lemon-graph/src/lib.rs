//! Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).
//!
//! # Usage
//!
//! ```
//! use lemon_graph::{Engine, Graph, GraphEdge, GraphNode, nodes::Delay};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut graph = Graph::new();
//!
//!     // Create a trigger, an entry point into the graph.
//!     let trigger = graph.add_node(GraphNode::Trigger("start".to_string()));
//!
//!     // Create a delay node, which will wait for 0.1 seconds before continuing to the next node.
//!     let delay = graph.add_node(Delay::new(0.1).into());
//!
//!     // Connect the trigger to the delay node.
//!     graph.add_edge(trigger, delay, GraphEdge::Flow);
//!
//!     // Create an execution engine and execute the graph.
//!     let mut engine = Engine(graph);
//!     engine.execute("start").await;
//! }
//! ```

use std::collections::HashMap;

use nodes::{AsyncNode, SyncNode};
use petgraph::graph::DiGraph;

pub mod engine;
pub mod nodes;

pub use engine::Engine;

pub enum GraphNode {
    Async(Box<dyn AsyncNode>),
    Sync(Box<dyn SyncNode>),
    Trigger(String),
}

pub enum GraphEdge {
    /// Data I/O between nodes
    Data { key: String, data: Data },
    /// Execution flow between nodes
    Flow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Bool(bool),
    Bytes(Vec<u8>),
    F32(f32),
    HashMap(HashMap<String, Data>),
    ISize(isize),
    String(String),
    USize(usize),
    Vec(Vec<Data>),
}

pub type Graph = DiGraph<GraphNode, GraphEdge>;
