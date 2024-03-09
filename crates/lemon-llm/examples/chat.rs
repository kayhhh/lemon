use std::sync::Arc;

use lemon_graph::{
    nodes::{CallbackNode, NodeWrapper, PromptNode},
    Executor, Value,
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
        model: OllamaModel::Mistral,
        ..Default::default()
    });
    let llm = LlmNode::new(&mut graph, LlmWeight::new(backend));

    // Create a prompt node to get user input.
    let prompt = PromptNode::new(&mut graph);

    // Create a callback node to format the LLM output.
    let format = CallbackNode::new(&mut graph, |input| {
        let input = match input {
            lemon_graph::Value::String(value) => value,
            _ => panic!("Invalid input"),
        };

        let out = format!("> {}", input);

        Value::String(out)
    });

    // Create the execution flow.
    llm.run_after(&mut graph, prompt.0);
    format.run_after(&mut graph, llm.0);
    prompt.run_after(&mut graph, format.0);

    // Connect the LLM output -> format input.
    let format_input = format.input(&graph).unwrap();
    let llm_output = llm.output(&graph).unwrap();
    format_input.set_input(&mut graph, Some(llm_output));

    // Connect the formatted output -> prompt input.
    let prompt_input = prompt.input(&graph).unwrap();
    let format_output = format.output(&graph).unwrap();
    prompt_input.set_input(&mut graph, Some(format_output));

    // Connect the prompt output -> LLM input.
    let prompt_output = prompt.output(&graph).unwrap();
    let llm_input = llm.input(&graph).unwrap();
    llm_input.set_input(&mut graph, Some(prompt_output));

    // Execute the graph.
    Executor::execute(&mut graph, prompt.0).await.unwrap();
}
