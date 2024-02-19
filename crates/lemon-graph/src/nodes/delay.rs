use std::{collections::HashMap, future::Future, time::Duration};

use crate::GraphNode;

use super::{AsyncNode, Data, NodeInput, SyncNode};

#[derive(Default)]
pub struct Delay {
    /// Duration to wait in seconds.
    pub duration: f32,
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
        Box::new(tokio::time::sleep(duration))
    }
}

impl SyncNode for Delay {
    fn run(&self) {
        let duration = Duration::from_secs_f32(self.duration);
        std::thread::sleep(duration);
    }
}

impl From<Delay> for GraphNode {
    fn from(node: Delay) -> Self {
        GraphNode::Sync(Box::new(node))
    }
}
