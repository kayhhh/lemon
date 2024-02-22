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
