//! Graph execution engine.

use std::collections::HashMap;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::{Data, Graph, GraphEdge};

#[derive(Default)]
pub struct Engine {
    pub graph: Graph,
}

impl Engine {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub async fn step(&self) {}
}

fn read_input(node_idx: NodeIndex, graph: &Graph) -> HashMap<String, Data> {
    graph
        .edges_directed(node_idx, Direction::Incoming)
        .fold(HashMap::new(), |mut acc, edge| {
            match &graph[edge.id()] {
                GraphEdge::Data { key, data } => {
                    acc.insert(key.clone(), data.clone());
                }
                _ => {}
            }

            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::{nodes::delay::Delay, Graph};

    #[test]
    fn test_engine() {
        let mut graph = Graph::default();
        let idx = graph.add_node(Delay { duration: 1.0 }.into());
    }
}
