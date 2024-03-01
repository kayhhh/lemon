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
//! }
//! ```

use nodes::Node;
use petgraph::graph::DiGraph;

pub mod nodes;
mod value;

pub use value::Value;

pub enum GraphEdge {
    /// Data I/O between nodes
    Data(usize),
    /// Execution flow between nodes
    Flow,
}

pub type Graph = DiGraph<Box<dyn Node>, GraphEdge>;
