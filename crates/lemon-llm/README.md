# lemon-llm

<!-- cargo-rdme start -->

LLM nodes for [lemon-graph](https://github.com/unavi-xyz/lemon/tree/main/crates/lemon-graph).

### Usage

In this example we create a basic chat app.
We take user input, send it to an LLM, print the response, and repeat.

```rust
use std::sync::Arc;

use lemon_graph::{Graph, Executor, nodes::{NodeWrapper, PromptNode}};
use lemon_llm::{ollama::{OllamaBackend, OllamaModel}, LlmBackend, LlmNode, LlmWeight};

#[tokio::main]
async fn main() {
   let mut graph = Graph::default();

   // Create a new Ollama backend.
   let backend = Arc::new(OllamaBackend {
       model: OllamaModel::Mistral,
       ..Default::default()
   });

   // Create an LLM node, using our Ollama backend.
   let llm = LlmNode::new(&mut graph, LlmWeight::new(backend));

   // Create a prompt node to get user input.
   let prompt = PromptNode::new(&mut graph);

   // Run each node in a cycle.
   llm.run_after(&mut graph, prompt.0);
   prompt.run_after(&mut graph, llm.0);

   // Connect the LLM output -> prompt input.
   let llm_output = llm.output(&graph).unwrap();
   let prompt_input = prompt.input(&graph).unwrap();
   prompt_input.set_input(&mut graph, Some(llm_output));

   // Connect the prompt output -> LLM input.
   let prompt_output = prompt.output(&graph).unwrap();
   let llm_input = llm.input(&graph).unwrap();
   llm_input.set_input(&mut graph, Some(prompt_output));

   // Execute the graph.
   // Executor::execute(&mut graph, prompt.0).await.unwrap();
}
```

<!-- cargo-rdme end -->
