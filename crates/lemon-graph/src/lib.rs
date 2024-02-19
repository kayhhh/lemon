use nodes::{AsyncNode, SyncNode};
use petgraph::graph::DiGraph;

pub mod engine;
pub mod nodes;

pub enum GraphNode {
    Async(Box<dyn AsyncNode>),
    Sync(Box<dyn SyncNode>),
}

pub enum GraphEdge {
    /// Data I/O between nodes
    Data { key: String, data: Data },
    /// Execution flow between nodes
    Flow,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Data {
    Bool(bool),
    Bytes(Vec<u8>),
    F32(f32),
    ISize(isize),
    List(Vec<Data>),
    String(String),
    USize(usize),
}

pub type Graph = DiGraph<GraphNode, GraphEdge>;
