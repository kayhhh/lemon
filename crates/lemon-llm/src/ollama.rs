use std::future::Future;

use crate::LlmBackend;

pub struct OllamaBackend;

impl LlmBackend for OllamaBackend {
    fn generate(&self, prompt: &str) -> Box<dyn Future<Output = ()> + Unpin> {
        Box::new(Box::pin(async move {
            let _ = prompt;
        }))
    }
}
