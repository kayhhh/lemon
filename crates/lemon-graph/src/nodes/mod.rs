use std::future::Future;
use thiserror::Error;

mod log;
pub mod util;

pub use log::Log;

use crate::Value;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Missing input at index {0}")]
    MissingInput(usize),
    #[error("Conversion error, got {0:?}")]
    ConversionError(Value),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait AsyncNode {
    fn run(
        &self,
        inputs: Vec<Value>,
    ) -> Box<dyn Future<Output = Result<Vec<Value>, NodeError>> + Unpin>;
}

pub trait SyncNode {
    fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>, NodeError>;
}

#[derive(Debug, Error)]
pub enum SetStoreError {
    #[error("No store found")]
    NoStore,
}
