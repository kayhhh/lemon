use petgraph::graph::NodeIndex;
use std::future::Future;
use thiserror::Error;

mod log;
pub mod util;

use crate::{Graph, Value};

use self::util::{input_stores, next_nodes, output_stores, previous_nodes};

#[derive(Debug, Error)]
pub enum NodeError {
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

pub trait NodeInterface: Copy + Into<NodeIndex> {
    fn previous_nodes(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        previous_nodes(self.into(), graph)
    }

    fn next_nodes(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        next_nodes(self.into(), graph)
    }

    fn input_stores(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        input_stores(self.into(), graph)
    }

    fn output_stores(self, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
        output_stores(self.into(), graph)
    }
}

#[derive(Debug, Error)]
pub enum SetStoreError {
    #[error("No store found")]
    NoStore,
}
