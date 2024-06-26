use petgraph::graph::NodeIndex;
use tracing::info;

use crate::{
    nodes::{GetStoreError, Node, NodeError, Store, SyncNode},
    Graph, GraphEdge, GraphNode, Value,
};

/// Logs a provided message.
#[derive(Debug, Clone, Copy)]
pub struct LogNode(pub NodeIndex);

impl From<LogNode> for NodeIndex {
    fn from(value: LogNode) -> Self {
        value.0
    }
}

impl Node for LogNode {}

impl LogNode {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(GraphNode::SyncNode(Box::new(LogWeight)));

        let input = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, index, GraphEdge::DataMap(0));

        Self(index)
    }

    pub fn message(&self, graph: &Graph) -> Result<Store, GetStoreError> {
        self.input_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }
}

struct LogWeight;

impl SyncNode for LogWeight {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError> {
        let input = inputs
            .first()
            .ok_or(NodeError::InternalError("No input".to_string()))?;

        info!("{}", input);

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::Executor;

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
        let mut graph = Graph::default();
        let log = LogNode::new(&mut graph);

        let message = log.message(&graph).unwrap();
        message.set_value(&mut graph, "Hello, world!".to_string().into());

        Executor::execute(&mut graph, log.0).await.unwrap();

        assert!(logs_contain("Hello, world!"));
    }
}
