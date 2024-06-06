use lemon_graph::nodes::Node;
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Copy)]
pub struct QdrantNode(pub NodeIndex);

impl From<QdrantNode> for NodeIndex {
    fn from(value: QdrantNode) -> Self {
        value.0
    }
}

impl Node for QdrantNode {}

#[cfg(test)]
mod tests {
    use qdrant_client::{client::QdrantClient, qdrant::CreateCollection};

    async fn fresh_client() -> QdrantClient {
        let client = QdrantClient::new(None).unwrap();

        let collections = client.list_collections().await.unwrap();

        for collection in collections.collections {
            client.delete_collection(&collection.name).await.unwrap();
        }

        client
    }

    #[tokio::test]
    async fn test_qdrant_client() {
        let client = fresh_client().await;

        client
            .create_collection(&CreateCollection {
                collection_name: "test".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
    }
}
