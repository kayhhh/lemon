//! Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).
//!
//! # Usage
//!
//! ```
//! use lemon_graph::{Executor, Graph, Value, nodes::{CallbackNode, LogNode, Node}};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut graph = Graph::default();
//!
//!     // Create a callback node to generate some data.
//!     let callback = CallbackNode::new(&mut graph, |_| {
//!         let time = std::time::SystemTime::now();
//!         let time = time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
//!         Value::String(format!("Current time: {}", time))
//!     });
//!
//!     // Create a log node.
//!     let log = LogNode::new(&mut graph);
//!     log.run_after(&mut graph, callback.0);
//!
//!     // Set the log message to the callback output.
//!     let message = log.message(&graph).unwrap();
//!     let callback_output = callback.output(&graph).unwrap();
//!     message.set_input(&mut graph, Some(callback_output));
//!
//!     // Execute the graph.
//!     Executor::execute(&mut graph, callback.0).await.unwrap();
//! }
//! ```

use nodes::{AsyncNode, SyncNode};
use petgraph::graph::DiGraph;

mod execution;
pub mod nodes;
mod value;

pub use execution::*;
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
    /// Used as an intermediary store for data between nodes.
    Store(Value),
}

pub type Graph = DiGraph<GraphNode, GraphEdge>;
