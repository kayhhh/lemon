# lemon-graph

<!-- cargo-rdme start -->

Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).

## Usage

```rust
use lemon_graph::{Graph, Executor, nodes::{NodeWrapper, LogNode}};

#[tokio::main]
async fn main() {
    let mut graph = Graph::new();

    // Create a log node and set its message.
    let log = LogNode::new(&mut graph);
    let message = log.message(&graph).unwrap();
    message.set_value(&mut graph, "Hello, world!".to_string().into());

    // Create a second log node to run after the first.
    let log_2 = LogNode::new(&mut graph);
    log_2.run_after(&mut graph, log.0);

    // Use the first log's message as input to the second log's message.
    let message_2 = log_2.message(&graph).unwrap();
    message_2.set_input(&mut graph, Some(message));

    // Execute the graph.
    Executor::execute(&mut graph, log.0).await.unwrap();
}
```

<!-- cargo-rdme end -->
