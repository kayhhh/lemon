use std::{collections::HashMap, future::Future, time::Duration};

use petgraph::graph::NodeIndex;
use tokio::time::sleep;

use super::{AsyncNode, Data, Node, NodeInput};

pub struct Delay {
    pub idx: NodeIndex,
    /// Duration to wait in seconds.
    pub duration: f32,
}

impl Delay {
    pub fn new(idx: NodeIndex) -> Self {
        Self { idx, duration: 0.0 }
    }
}

impl Node for Delay {
    fn idx(&self) -> NodeIndex {
        self.idx
    }
}

impl NodeInput for Delay {
    fn process_input(&mut self, input: HashMap<String, Data>) {
        if let Some(Data::F32(duration)) = input.get("duration") {
            self.duration = *duration;
        }
    }
}

impl AsyncNode for Delay {
    fn run(&self) -> Box<dyn Future<Output = ()>> {
        let duration = Duration::from_secs_f32(self.duration);
        Box::new(sleep(duration))
    }
}
