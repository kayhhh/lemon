use std::{future::Future, time::Duration};

use crate::Value;

use super::{Node, TypedNode};

pub struct Delay;

impl TypedNode<f32, f32> for Delay {}

impl Node for Delay {
    fn run(&mut self, input: Value) -> Box<dyn Future<Output = Value> + Unpin> {
        let duration = match input {
            Value::F32(duration) => duration,
            _ => panic!("Expected f32"),
        };

        let start = tokio::time::Instant::now();

        Box::new(Box::pin(async move {
            tokio::time::sleep(Duration::from_secs_f32(duration)).await;
            let elapsed = start.elapsed().as_secs_f32();
            Value::F32(elapsed)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay() {
        let mut delay = Delay;

        let value = 0.1;
        let out = delay.run_typed(value).await.unwrap();
        assert!(out > value);

        let value = Value::F32(0.2);
        let out = delay.run(value.clone()).await;
        assert!(out > value);
    }
}
