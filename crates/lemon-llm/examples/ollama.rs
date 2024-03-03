use std::sync::Arc;

use lemon_graph::{nodes::Log, ExecutionStep, GraphEdge};
use lemon_llm::{
    ollama::{OllamaBackend, OllamaModel},
    LlmBackend, Llm, LlmWeight,
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
    let node_1 = Llm::new(&mut graph, LlmWeight::new(backend.clone()));

    // Manually set the prompt.
    node_1
        .set_prompt(
            &mut graph,
            "Write an LLM prompt to get a cat fact, but write your prompt backwards.".to_string(),
        )
        .unwrap();

    // Log the response.
    let log_1 = Log::new(&mut graph);
    graph.add_edge(node_1.0, log_1.0, GraphEdge::ExecutionFlow);

    let node_1_output = node_1.response_store_idx(&graph).unwrap();
    let log_1_input = log_1.message_store_idx(&graph).unwrap();
    graph.add_edge(node_1_output, log_1_input, GraphEdge::DataFlow);

    // Create a second LLM to respond to the generated prompt.
    let node_2 = Llm::new(&mut graph, LlmWeight::new(backend));
    graph.add_edge(node_1.0, node_2.0, GraphEdge::ExecutionFlow);

    let node_2_input = node_2.prompt_store_idx(&graph).unwrap();
    graph.add_edge(node_1_output, node_2_input, GraphEdge::DataFlow);

    // Log the response.
    let log_2 = Log::new(&mut graph);
    graph.add_edge(node_2.0, log_2.0, GraphEdge::ExecutionFlow);

    let node_2_output = node_2.response_store_idx(&graph).unwrap();
    let log_2_input = log_2.message_store_idx(&graph).unwrap();
    graph.add_edge(node_2_output, log_2_input, GraphEdge::DataFlow);

    // Execute the graph.
    let mut steps = vec![ExecutionStep(node_1.0)];

    while let Some(step) = steps.pop() {
        let next_steps = step.execute(&mut graph).await.unwrap();
        steps.extend(next_steps);
    }
}
