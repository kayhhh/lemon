use std::future::Future;
use thiserror::Error;

mod constant;
mod delay;
mod log;

pub use constant::Constant;
pub use delay::Delay;
pub use log::Log;

use crate::Value;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Conversion error, got {0:?}")]
    ConversionError(Value),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait Node {
    fn run(&mut self, input: Value) -> Box<dyn Future<Output = Result<Value, NodeError>> + Unpin>;
}

pub trait TypedNode<I: Into<Value>, O: TryFrom<Value>>: Node {
    fn run_typed(&mut self, input: I) -> impl Future<Output = Result<O, NodeError>> {
        async move {
            let input = input.into();
            let output_value = self.run(input.clone()).await?;
            O::try_from(output_value.clone()).map_err(|_| NodeError::ConversionError(output_value))
        }
    }
}
