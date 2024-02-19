use std::{collections::HashMap, future::Future};

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::Graph;

pub mod delay;

pub trait Node {
    fn idx(&self) -> NodeIndex;
}

pub trait NodeInput: Node {
    /// Read input edges from the graph.
    fn read_input(&self, graph: &Graph) -> HashMap<String, Data> {
        graph.edges_directed(self.idx(), Direction::Incoming).fold(
            HashMap::new(),
            |mut acc, edge| {
                match &graph[edge.id()] {
                    GraphEdge::Data { key, data } => {
                        acc.insert(key.clone(), data.clone());
                    }
                    _ => {}
                }

                acc
            },
        )
    }

    /// Process input from the graph.
    /// Called before running the node.
    fn process_input(&mut self, input: HashMap<String, Data>);
}

pub trait AsyncNode: Node {
    fn run(&self) -> Box<dyn Future<Output = ()>>;
}

pub trait SyncNode: Node {
    fn run(&self);
}

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
