use lemon_graph::{Data, Engine, GraphEdge, GraphNode};
use lemon_llm::{
    ollama::{OllamaBackend, OllamaModel},
    LlmBackend, LlmNode,
};
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

    // Use the backend within a graph.
    let mut engine = Engine::default();

    let llm = engine.0.add_node(
        LlmNode {
            backend: backend.into(),
            prompt: "Tell me a short cat fact!".to_string(),
        }
        .into(),
    );

    let trigger = engine.0.add_node(GraphNode::Trigger("start".to_string()));
    engine.0.add_edge(trigger, llm, GraphEdge::Flow);

    let result = engine
        .execute("start")
        .await
        .expect("Failed to execute graph");

    let result = match result {
        Data::String(s) => s,
        _ => panic!("Unexpected result"),
    };

    info!("{:?}", result);
}
