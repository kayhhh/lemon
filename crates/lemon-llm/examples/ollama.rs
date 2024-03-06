use std::sync::Arc;

use lemon_graph::{
    nodes::{LogNode, NodeWrapper},
    Executor,
};
use lemon_llm::{
    ollama::{OllamaBackend, OllamaModel},
    LlmBackend, LlmNode, LlmWeight,
};
use petgraph::Graph;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a new Ollama backend.
    // Assumes you have the Ollama server running.
    let backend = OllamaBackend {
        model: OllamaModel::Mistral7B,
        ..Default::default()
    };

    // Generate a response directly from the backend.
    // This will automatically download the model if it is not already present.
    let response = backend
        .generate("Tell me a short joke.")
        .await
        .expect("Failed to send message");

    info!("{}", response);

    // Or you can use the backend within a graph.
    let mut graph = Graph::default();

    // Create an initial LLM to generate a prompt for the next LLM.
    let backend = Arc::new(backend);
    let llm_1 = LlmNode::new(&mut graph, LlmWeight::new(backend.clone()));

    let prompt = llm_1.prompt(&graph).unwrap();
    prompt.set_value(
        &mut graph,
        "Write an LLM prompt to get a cat fact, but write your prompt backwards."
            .to_string()
            .into(),
    );

    let response = llm_1.response(&graph).unwrap();

    // Log the response.
    {
        let log = LogNode::new(&mut graph);
        log.run_after(&mut graph, llm_1.0);

        let message = log.message(&graph).unwrap();
        message.set_input(&mut graph, Some(response));
    }

    // Create a second LLM to respond to the generated prompt.
    let llm_2 = LlmNode::new(&mut graph, LlmWeight::new(backend));
    llm_2.run_after(&mut graph, llm_1.0);

    let prompt = llm_2.prompt(&graph).unwrap();
    prompt.set_input(&mut graph, Some(response));

    let response = llm_2.response(&graph).unwrap();

    // Log the response.
    {
        let log = LogNode::new(&mut graph);
        log.run_after(&mut graph, llm_2.0);

        let message = log.message(&graph).unwrap();
        message.set_input(&mut graph, Some(response));
    }

    // Execute the graph.
    Executor::execute(&mut graph, llm_1.0).await.unwrap();
}
