use std::sync::Arc;

use lemon_graph::{
    nodes::{Log, NodeWrapper},
    ExecutionStep,
};
use lemon_llm::{
    ollama::{OllamaBackend, OllamaModel},
    Llm, LlmBackend, LlmWeight,
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
    let mut graph = Graph::new();

    // Create an initial LLM to generate a prompt for the next LLM.
    let backend = Arc::new(backend);
    let llm1 = Llm::new(&mut graph, LlmWeight::new(backend.clone()));

    let prompt = llm1.prompt(&graph).unwrap();
    prompt.set_value(
        &mut graph,
        "Write an LLM prompt to get a cat fact, but write your prompt backwards."
            .to_string()
            .into(),
    );

    let response = llm1.response(&graph).unwrap();

    // Log the response.
    {
        let log = Log::new(&mut graph);
        log.run_after(&mut graph, llm1.0);

        let log_message = log.message(&graph).unwrap();
        log_message.set_input(&mut graph, Some(response));
    }

    // Create a second LLM to respond to the generated prompt.
    let llm2 = Llm::new(&mut graph, LlmWeight::new(backend));
    llm2.run_after(&mut graph, llm1.0);

    let prompt = llm2.prompt(&graph).unwrap();
    prompt.set_input(&mut graph, Some(response));

    let response = llm2.response(&graph).unwrap();

    // Log the response.
    {
        let log = Log::new(&mut graph);
        log.run_after(&mut graph, llm2.0);

        let log_message = log.message(&graph).unwrap();
        log_message.set_input(&mut graph, Some(response));
    }

    // Execute the graph.
    let mut steps = vec![ExecutionStep(llm1.0)];

    while let Some(step) = steps.pop() {
        let next_steps = step.execute(&mut graph).await.unwrap();
        steps.extend(next_steps);
    }
}
