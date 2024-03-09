# lemon-graph

<!-- cargo-rdme start -->

Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).

## Usage

```rust
use lemon_graph::{Executor, Graph, Value, nodes::{CallbackNode, LogNode, NodeWrapper}};

#[tokio::main]
async fn main() {
    let mut graph = Graph::default();

    // Create a callback node to generate some data.
    let callback = CallbackNode::new(&mut graph, |_| {
        let time = std::time::SystemTime::now();
        let time = time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        Value::String(format!("Current time: {}", time))
    });

    // Create a log node.
    let log = LogNode::new(&mut graph);
    log.run_after(&mut graph, callback.0);

    // Set the log message to the callback output.
    let message = log.message(&graph).unwrap();
    let callback_output = callback.output(&graph).unwrap();
    message.set_input(&mut graph, Some(callback_output));

    // Execute the graph.
    Executor::execute(&mut graph, callback.0).await.unwrap();
}
```

<!-- cargo-rdme end -->
