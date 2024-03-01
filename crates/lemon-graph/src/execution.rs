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
