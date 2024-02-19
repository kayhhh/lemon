use nodes::{GraphEdge, GraphNode};
use petgraph::graph::DiGraph;

pub mod nodes;

pub type Graph = DiGraph<GraphNode, GraphEdge>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine() {
        let mut graph = Graph::default();
    }
}
