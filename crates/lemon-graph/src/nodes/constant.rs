use std::future::Future;

use crate::Value;

use super::{Node, TypedNode};

pub struct Constant<T: Clone + Into<Value> + TryFrom<Value>>(T);

impl<T: Clone + Into<Value> + TryFrom<Value>> TypedNode<(), T> for Constant<T> {}

impl<T: Clone + Into<Value> + TryFrom<Value>> Node for Constant<T> {
    fn run(&mut self, _: Value) -> Box<dyn Future<Output = Value> + Unpin> {
        let value = self.0.clone().into();
        Box::new(Box::pin(async move { value }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_constant() {
        let value: usize = 42;
        let mut constant = Constant(value);

        let out = constant.run_typed(()).await.unwrap();
        assert_eq!(out, value);

        let out = constant.run(Value::None).await;
        assert_eq!(out, Value::USize(42));
    }
}
