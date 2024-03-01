# lemon-graph

<!-- cargo-rdme start -->

Async directed computation graphs.

## Usage

```rust
use lemon_graph::{Engine, Graph, GraphEdge, GraphNode, nodes::Delay};

#[tokio::main]
async fn main() {
    let mut graph = Graph::new();

    // Create a trigger, an entry point into the graph.
    let trigger = graph.add_node(GraphNode::Trigger("start".to_string()));

    // Create a delay node, which will wait for 0.1 seconds before continuing to the next node.
    let delay = graph.add_node(Delay::new(0.1).into());

    // Connect the trigger to the delay node.
    graph.add_edge(trigger, delay, GraphEdge::Flow);

    // Create an execution engine and execute the graph.
    let mut engine = Engine(graph);
    engine.execute("start").await;
}
```

<!-- cargo-rdme end -->
