use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};
use thiserror::Error;

use crate::{nodes::NodeError, Graph, GraphEdge, GraphNode};

pub struct ExecutionStep(pub NodeIndex);

#[derive(Debug, Error)]
pub enum ExecutionStepError {
    #[error("No weight")]
    NoWeight,
    #[error("Invalid weight")]
    InvalidWeight,
    #[error(transparent)]
    NodeError(#[from] NodeError),
}

impl ExecutionStep {
    pub async fn execute<'a>(
        &self,
        graph: &'a mut Graph,
    ) -> Result<impl Iterator<Item = ExecutionStep> + 'a, ExecutionStepError> {
        // Read inputs
        let mut inputs = graph
            .edges_directed(self.0, Direction::Incoming)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::DataMap(data_idx) => Some((*data_idx, edge.source())),
                _ => None,
            })
            .map(|(data_idx, source_idx)| {
                let node = graph
                    .node_weight(source_idx)
                    .ok_or(ExecutionStepError::NoWeight)?;

                match node {
                    GraphNode::Store(value) => Ok((data_idx, value.clone())),
                    _ => Err(ExecutionStepError::InvalidWeight),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        inputs.sort_by_key(|(idx, _)| *idx);

        let inputs = inputs.into_iter().map(|(_, value)| value).collect();

        // Execute node
        let node = graph
            .node_weight(self.0)
            .ok_or(ExecutionStepError::NoWeight)?;

        let res = match node {
            GraphNode::AsyncNode(node) => node.run(inputs).await?,
            GraphNode::SyncNode(node) => node.run(inputs)?,
            _ => return Err(ExecutionStepError::InvalidWeight),
        };

        // Write outputs
        let outputs = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::DataMap(data_idx) => Some((edge.target(), *data_idx)),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (i, value) in res.into_iter().enumerate() {
            let (store_idx, _) = match outputs.iter().find(|(_, idx)| *idx == i) {
                Some(output) => output,
                None => continue,
            };

            graph[*store_idx] = GraphNode::Store(value);
        }

        // Get next steps
        Ok(graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::ExecutionFlow => Some(ExecutionStep(edge.target())),
                _ => None,
            }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{AsyncNode, SyncNode},
        Value,
    };

    use super::*;

    struct TestSync;

    impl SyncNode for TestSync {
        fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError> {
            Ok(inputs)
        }
    }

    struct TestAsync;

    impl AsyncNode for TestAsync {
        fn run(
            &self,
            inputs: Vec<Value>,
        ) -> Box<dyn std::future::Future<Output = Result<Vec<Value>, NodeError>> + Unpin> {
            Box::new(Box::pin(async move { Ok(inputs) }))
        }
    }

    #[tokio::test]
    async fn test_sync_execution() {
        let mut graph = Graph::new();

        let input = graph.add_node(GraphNode::Store(Value::String("Hello, world!".to_string())));
        let node = graph.add_node(GraphNode::SyncNode(Box::new(TestSync)));
        let output = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, node, GraphEdge::DataMap(0));
        graph.add_edge(node, output, GraphEdge::DataMap(0));

        let step = ExecutionStep(node);
        let next_steps = step.execute(&mut graph).await.unwrap().collect::<Vec<_>>();

        assert_eq!(next_steps.len(), 0);

        let output_value = graph.node_weight(output).unwrap();
        let output_value = match output_value {
            GraphNode::Store(value) => value,
            _ => panic!(),
        };
        assert_eq!(output_value, &Value::String("Hello, world!".to_string()));
    }

    #[tokio::test]
    async fn test_async_execution() {
        let mut graph = Graph::new();

        let input = graph.add_node(GraphNode::Store(Value::String("Hello, world!".to_string())));
        let node = graph.add_node(GraphNode::AsyncNode(Box::new(TestAsync)));
        let output = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, node, GraphEdge::DataMap(0));
        graph.add_edge(node, output, GraphEdge::DataMap(0));

        let step = ExecutionStep(node);
        let next_steps = step.execute(&mut graph).await.unwrap().collect::<Vec<_>>();

        assert_eq!(next_steps.len(), 0);

        let output_value = graph.node_weight(output).unwrap();
        let output_value = match output_value {
            GraphNode::Store(value) => value,
            _ => panic!(),
        };
        assert_eq!(output_value, &Value::String("Hello, world!".to_string()));
    }
}
