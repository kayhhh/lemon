use std::future::Future;

use crate::LlmBackend;

pub struct ReplicateBackend;

impl LlmBackend for ReplicateBackend {
    fn generate(&self, prompt: &str) -> Box<dyn Future<Output = ()> + Unpin> {
        Box::new(Box::pin(async move {
            let _ = prompt;
        }))
    }
}
