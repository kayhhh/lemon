use petgraph::graph::NodeIndex;

use crate::{Graph, GraphEdge, GraphNode, Value};

use super::{GetStoreError, NodeError, NodeWrapper, StoreWrapper, SyncNode};

/// Callbacks a provided message.
#[derive(Debug, Clone, Copy)]
pub struct CallbackNode(pub NodeIndex);

impl From<CallbackNode> for NodeIndex {
    fn from(value: CallbackNode) -> Self {
        value.0
    }
}

impl NodeWrapper for CallbackNode {}

impl CallbackNode {
    pub fn new(graph: &mut Graph, cb: impl Fn(Value) -> Value + 'static) -> Self {
        let index = graph.add_node(GraphNode::SyncNode(Box::new(CallbackWeight {
            cb: Box::new(cb),
        })));

        let input = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(input, index, GraphEdge::DataMap(0));

        let output = graph.add_node(GraphNode::Store(Value::String(Default::default())));
        graph.add_edge(index, output, GraphEdge::DataMap(0));

        Self(index)
    }

    pub fn input(&self, graph: &Graph) -> Result<StoreWrapper, GetStoreError> {
        self.input_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }

    pub fn output(&self, graph: &Graph) -> Result<StoreWrapper, GetStoreError> {
        self.output_stores(graph)
            .next()
            .ok_or(GetStoreError::NoStore)
    }
}

struct CallbackWeight {
    cb: Box<dyn Fn(Value) -> Value>,
}

impl SyncNode for CallbackWeight {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError> {
        let input = inputs
            .first()
            .ok_or(NodeError::InternalError("No input".to_string()))?;

        let output = (self.cb)(input.clone());

        Ok(vec![output])
    }
}

#[cfg(test)]
mod tests {

    use crate::Executor;

    use super::*;

    #[test]
    fn test_callback_weight() {
        let weight = CallbackWeight {
            cb: Box::new(|input| {
                let input = match input {
                    Value::String(value) => value,
                    _ => panic!("Invalid input"),
                };

                let input = input.to_uppercase();

                Value::String(input)
            }),
        };

        let out = weight
            .run(vec!["Hello, world!".to_string().into()])
            .unwrap();

        assert_eq!(out, vec!["HELLO, WORLD!".to_string().into()]);
    }

    #[tokio::test]
    async fn test_callback() {
        let mut graph = Graph::default();

        let callback = CallbackNode::new(&mut graph, |input| {
            let value = match input {
                Value::String(value) => value,
                _ => panic!("Invalid input"),
            };

            Value::String(value.to_uppercase())
        });

        let input = callback.input(&graph).unwrap();
        input.set_value(&mut graph, "Hello, world!".to_string().into());

        let callback_2 = CallbackNode::new(&mut graph, |input| {
            let value = match &input {
                Value::String(value) => value,
                _ => panic!("Invalid input"),
            };

            assert_eq!(value, "HELLO, WORLD!");

            input
        });

        callback_2.run_after(&mut graph, callback.0);

        let output = callback.output(&graph).unwrap();
        let input = callback_2.input(&graph).unwrap();
        input.set_input(&mut graph, Some(output));

        Executor::execute(&mut graph, callback.0).await.unwrap();
    }
}
