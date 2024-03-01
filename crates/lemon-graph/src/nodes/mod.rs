use std::future::Future;
use thiserror::Error;

mod constant;
mod delay;
mod log;

pub use constant::Constant;
pub use delay::Delay;
pub use log::Log;

use crate::Value;

pub trait Node {
    fn run(&mut self, input: Value) -> Box<dyn Future<Output = Value> + Unpin>;
}

#[derive(Debug, Error)]
pub enum RunTypedError {
    #[error("Conversion error, expected {0:?}, got {1:?}")]
    ConversionError(Value, Value),
}

pub trait TypedNode<I: Into<Value>, O: TryFrom<Value>>: Node {
    fn run_typed(&mut self, input: I) -> impl Future<Output = Result<O, RunTypedError>> {
        async move {
            let input = input.into();
            let output_value = self.run(input.clone()).await;
            O::try_from(output_value.clone())
                .map_err(|_| RunTypedError::ConversionError(input, output_value))
        }
    }
}
