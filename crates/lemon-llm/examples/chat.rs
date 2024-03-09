use std::sync::Arc;

use lemon_graph::{
    nodes::{NodeWrapper, PromptNode},
    Executor,
};
use lemon_llm::{
    ollama::{OllamaBackend, OllamaModel},
    LlmNode, LlmWeight,
};
use petgraph::Graph;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut graph = Graph::default();

    // Create an LLM node.
    let backend = Arc::new(OllamaBackend {
        model: OllamaModel::Mistral7B,
        ..Default::default()
    });
    let llm = LlmNode::new(&mut graph, LlmWeight::new(backend.clone()));

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
    Executor::execute(&mut graph, prompt.0).await.unwrap();
}
