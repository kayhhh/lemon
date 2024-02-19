use std::{collections::HashMap, future::Future};

use crate::Data;

pub mod delay;

pub trait NodeInput {
    /// Process input from the graph.
    /// Called before running the node.
    fn process_input(&mut self, input: HashMap<String, Data>);
}

pub trait AsyncNode {
    fn run(&self) -> Box<dyn Future<Output = ()>>;
}

pub trait SyncNode {
    fn run(&self);
}
