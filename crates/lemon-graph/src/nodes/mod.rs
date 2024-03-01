use std::{collections::HashMap, future::Future};

use crate::Data;

mod delay;

pub use delay::Delay;

pub trait Node {
    /// Process input from the graph.
    /// Called before running the node.
    fn process_input(&mut self, input: HashMap<String, Data>) {
        let _ = input;
    }
}

pub trait AsyncNode: Node {
    fn run(&mut self) -> Box<dyn Future<Output = Option<Data>> + Unpin>;
}

pub trait SyncNode: Node {
    fn run(&mut self) -> Option<Data>;
}
