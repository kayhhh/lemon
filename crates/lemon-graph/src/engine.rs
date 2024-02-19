use std::collections::HashMap;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::{Data, Graph, GraphEdge, GraphNode};

#[derive(Default)]
pub struct Engine(pub Graph);

impl Engine {
    pub async fn execute(&mut self, trigger: &str) {
        let trigger_nodes: Vec<NodeIndex> = self
            .0
            .node_indices()
            .filter(|idx| match &self.0[*idx] {
                GraphNode::Trigger(name) => name == trigger,
                _ => false,
            })
            .collect();

        for node in trigger_nodes {
            let step = ExecutionStep { idx: node };

            let mut steps = vec![step];

            while !steps.is_empty() {
                let mut next_steps = Vec::new();
                for step in steps {
                    next_steps.extend(step.step(&mut self.0).await);
                }
                steps = next_steps;
            }
        }
    }
}

pub struct ExecutionStep {
    pub idx: NodeIndex,
}

impl ExecutionStep {
    pub fn new(idx: NodeIndex) -> Self {
        Self { idx }
    }

    pub async fn step(&self, graph: &mut Graph) -> Vec<ExecutionStep> {
        // Read input from the graph.
        let input = read_input(self.idx, graph);

        // Process input and run the node.
        match &mut graph[self.idx] {
            GraphNode::Async(node) => {
                node.process_input(input);
                node.run().await;
            }
            GraphNode::Sync(node) => {
                node.process_input(input);
                node.run();
            }
            _ => {
                return Vec::new();
            }
        }

        // Return the next steps.
        graph
            .edges_directed(self.idx, Direction::Outgoing)
            .filter_map(|edge| match &graph[edge.id()] {
                GraphEdge::Flow => Some(ExecutionStep::new(edge.target())),
                _ => None,
            })
            .collect()
    }
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
    use crate::nodes::delay::Delay;

    use super::*;

    #[tokio::test]
    async fn test_engine() {
        let mut graph = Graph::new();

        let node_1 = graph.add_node(Delay::new(0.1).into());
        let node_2 = graph.add_node(Delay::new(0.2).into());
        graph.add_edge(node_1, node_2, GraphEdge::Flow);

        let trigger = graph.add_node(GraphNode::Trigger("start".to_string()));
        graph.add_edge(trigger, node_1, GraphEdge::Flow);

        let mut engine = Engine(graph);
        engine.execute("start").await;
    }
}
