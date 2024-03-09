//! Memory nodes for [lemon-graph](https://github.com/unavi-xyz/lemon/tree/main/crates/lemon-graph).

use lemon_graph::nodes::NodeWrapper;
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Copy)]
pub struct MemoryNode(pub NodeIndex);

impl From<MemoryNode> for NodeIndex {
    fn from(value: MemoryNode) -> Self {
        value.0
    }
}

impl NodeWrapper for MemoryNode {}
