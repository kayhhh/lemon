# lemon-llm

<!-- cargo-rdme start -->

LLM nodes for [lemon-graph](https://github.com/unavi-xyz/lemon/tree/main/crates/lemon-graph).

### Usage

```rust
use std::sync::Arc;

use lemon_graph::{Graph, Executor, nodes::{NodeWrapper, LogNode}};
use lemon_llm::{ollama::{OllamaBackend, OllamaModel}, LlmBackend, LlmNode, LlmWeight};

#[tokio::main]
async fn main() {
   let mut graph = Graph::default();

   // Create a new Ollama backend.
   let backend = Arc::new(OllamaBackend {
       model: OllamaModel::Mistral7B,
       ..Default::default()
   });

   // Create an llm node, using our Ollama backend.
   let llm = LlmNode::new(&mut graph, LlmWeight::new(backend.clone()));

   // Set the prompt manually.
   let prompt = llm.prompt(&graph).unwrap();
   prompt.set_value(
        &mut graph,
        "Tell me your favorite lemon fact.".to_string().into(),
   );

   // Connect the reponse to a log node.
   let response = llm.response(&graph).unwrap();

   let log = LogNode::new(&mut graph);
   log.run_after(&mut graph, llm.0);

   let message = log.message(&graph).unwrap();
   message.set_input(&mut graph, Some(response));

   // Execute the graph.
   // Executor::execute(&mut graph, llm.0).await.unwrap();
}
```

<!-- cargo-rdme end -->
