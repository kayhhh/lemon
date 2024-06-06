use petgraph::graph::NodeIndex;

use crate::{
    nodes::{GetStoreError, Node, NodeError, Store, SyncNode},
    Graph, GraphEdge, GraphNode, Value,
};

#[derive(Debug, Clone, Copy)]
pub struct PromptNode(pub NodeIndex);

impl From<PromptNode> for NodeIndex {
    fn from(value: PromptNode) -> Self {
        value.0
    }
}

impl Node for PromptNode {}

impl PromptNode {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(GraphNode::SyncNode(Box::new(PromptWeight)));

        let input = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, index, GraphEdge::DataMap(0));

        let output = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(index, output, GraphEdge::DataMap(0));

        Self(index)
    }

    pub fn input(&self, graph: &Graph) -> Result<Store, GetStoreError> {
        self.input_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }

    pub fn output(&self, graph: &Graph) -> Result<Store, GetStoreError> {
        self.output_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }
}

struct PromptWeight;

impl SyncNode for PromptWeight {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError> {
        let input = inputs
            .first()
            .ok_or(NodeError::InternalError("No input".to_string()))?;

        let input_value = match input {
            Value::String(value) => value,
            _ => return Err(NodeError::ConversionError(input.clone())),
        };

        println!("{}", input_value);

        let mut output_value = String::new();

        std::io::stdin()
            .read_line(&mut output_value)
            .map_err(|error| NodeError::InternalError(error.to_string()))?;

        Ok(vec![output_value.trim().to_string().into()])
    }
}
