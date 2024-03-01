# lemon-graph

<!-- cargo-rdme start -->

Async directed computation graphs, using [petgraph](https://crates.io/crates/petgraph).

## Usage

```rust
use lemon_graph::{Graph, ExecutionStep, nodes::Log};

#[tokio::main]
async fn main() {
    let mut graph = Graph::new();

    // Create a log node and set its message.
    let log = Log::new(&mut graph);
    log.set_message(&mut graph, "Hello, world!".to_string());

    // Execute the graph.
    let step = ExecutionStep(log.0);
    step.execute(&mut graph).await.unwrap();
}
```

<!-- cargo-rdme end -->
