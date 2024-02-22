use std::collections::HashMap;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::{Data, Graph, GraphEdge, GraphNode};

/// Graph execution engine.
#[derive(Default)]
pub struct Engine(pub Graph);

impl Engine {
    pub async fn execute(&mut self, trigger: &str) -> Option<Data> {
        let trigger_nodes: Vec<NodeIndex> = self
            .0
            .node_indices()
            .filter(|idx| match &self.0[*idx] {
                GraphNode::Trigger(name) => name == trigger,
                _ => false,
            })
            .collect();

        let mut data = None;

        for node in trigger_nodes {
            let step = ExecutionStep { idx: node };

            let mut steps = vec![step];

            while !steps.is_empty() {
                let mut next_steps = Vec::new();
                for step in steps {
                    let out = step.step(&mut self.0).await;
                    data = out.0;
                    next_steps.extend(out.1);
                }
                steps = next_steps;
            }
        }

        data
    }
}

pub struct ExecutionStep {
    pub idx: NodeIndex,
}

impl ExecutionStep {
    pub fn new(idx: NodeIndex) -> Self {
        Self { idx }
    }

    pub async fn step(&self, graph: &mut Graph) -> (Option<Data>, Vec<ExecutionStep>) {
        // Read input from the graph.
        let input = read_input(self.idx, graph);

        // Process input and run the node.
        let data = match &mut graph[self.idx] {
            GraphNode::Async(node) => {
                node.process_input(input);
                node.run().await
            }
            GraphNode::Sync(node) => {
                node.process_input(input);
                node.run()
            }
            _ => {
                return (None, Vec::new());
            }
        };

        // Return the next steps.
        let next_steps = graph
            .edges_directed(self.idx, Direction::Outgoing)
            .filter_map(|edge| match &graph[edge.id()] {
                GraphEdge::Flow => Some(ExecutionStep::new(edge.target())),
                _ => None,
            })
            .collect();

        (data, next_steps)
    }
}

fn read_input(node_idx: NodeIndex, graph: &Graph) -> HashMap<String, Data> {
    graph
        .edges_directed(node_idx, Direction::Incoming)
        .fold(HashMap::new(), |mut acc, edge| {
            if let GraphEdge::Data { key, data } = &graph[edge.id()] {
                acc.insert(key.clone(), data.clone());
            }

            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::nodes::delay::Delay;

    use super::*;

    #[tokio::test]
    async fn basic_flow() {
        let mut graph = Graph::new();

        let trigger = graph.add_node(GraphNode::Trigger("start".to_string()));
        let node_1 = graph.add_node(Delay::new(0.1).into());
        let node_2 = graph.add_node(Delay::new(0.2).into());
        graph.add_edge(trigger, node_1, GraphEdge::Flow);
        graph.add_edge(node_1, node_2, GraphEdge::Flow);

        let mut engine = Engine(graph);
        engine.execute("start").await;
    }

    #[tokio::test]
    async fn multiple_trigger_graphs() {
        let mut graph = Graph::new();

        let trigger_1 = graph.add_node(GraphNode::Trigger("start".to_string()));
        let trigger_2 = graph.add_node(GraphNode::Trigger("start".to_string()));
        let node_1 = graph.add_node(Delay::new(0.1).into());
        let node_2 = graph.add_node(Delay::new(0.2).into());
        graph.add_edge(trigger_1, node_1, GraphEdge::Flow);
        graph.add_edge(trigger_2, node_2, GraphEdge::Flow);

        let mut engine = Engine(graph);
        engine.execute("start").await;
    }

    #[tokio::test]
    async fn multiple_outputs() {
        let mut graph = Graph::new();

        let trigger = graph.add_node(GraphNode::Trigger("start".to_string()));
        let node_1 = graph.add_node(Delay::new(0.1).into());
        let node_2 = graph.add_node(Delay::new(0.2).into());
        let node_3 = graph.add_node(Delay::new(0.3).into());
        graph.add_edge(trigger, node_1, GraphEdge::Flow);
        graph.add_edge(node_1, node_2, GraphEdge::Flow);
        graph.add_edge(node_1, node_3, GraphEdge::Flow);

        let mut engine = Engine(graph);
        engine.execute("start").await;
    }
}
