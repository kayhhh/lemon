use petgraph::graph::NodeIndex;
use tracing::info;

use crate::{Graph, GraphEdge, GraphNode, Value};

use super::{NodeError, NodeInterface, SetStoreError, SyncNode};

/// Provides a log value.
#[derive(Debug, Clone, Copy)]
pub struct Log(pub NodeIndex);

impl From<NodeIndex> for Log {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Log> for NodeIndex {
    fn from(log: Log) -> Self {
        log.0
    }
}

impl NodeInterface for Log {}

impl Log {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(GraphNode::SyncNode(Box::new(LogWeight)));

        let input = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, index, GraphEdge::DataMap(0));

        Self(index)
    }

    /// Sets the value of the input store.
    pub fn set_message(&self, graph: &mut Graph, message: String) -> Result<(), SetStoreError> {
        let input_idx = self
            .input_stores(graph)
            .next()
            .ok_or(SetStoreError::NoStore)?;
        graph[input_idx] = GraphNode::Store(Value::String(message));
        Ok(())
    }
}

pub struct LogWeight;

impl SyncNode for LogWeight {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError> {
        let input = inputs
            .first()
            .ok_or(NodeError::InternalError("No input".to_string()))?;

        let value = match input {
            Value::String(value) => value,
            _ => return Err(NodeError::ConversionError(input.clone())),
        };

        info!("{}", value);

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::execution::ExecutionStep;

    use super::*;

    #[test]
    #[traced_test]
    fn test_log_weight() {
        let weight = LogWeight;

        weight
            .run(vec!["Hello, world!".to_string().into()])
            .unwrap();

        assert!(logs_contain("Hello, world!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_log() {
        let mut graph = Graph::new();
        let log = Log::new(&mut graph);

        log.set_message(&mut graph, "Hello, world!".to_string())
            .unwrap();

        let step = ExecutionStep(log.0);

        let _ = step.execute(&mut graph).await.unwrap();

        assert!(logs_contain("Hello, world!"));
    }
}
