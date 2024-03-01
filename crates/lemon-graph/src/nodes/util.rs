use petgraph::{graph::NodeIndex, Direction};

use crate::{Graph, GraphNode};

pub fn previous_nodes(index: NodeIndex, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
    graph
        .neighbors_directed(index, Direction::Incoming)
        .filter(|idx| matches!(&graph[*idx], GraphNode::AsyncNode(_)))
}

pub fn next_nodes(index: NodeIndex, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
    graph
        .neighbors_directed(index, Direction::Outgoing)
        .filter(|idx| matches!(&graph[*idx], GraphNode::AsyncNode(_)))
}

pub fn input_stores(index: NodeIndex, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
    graph
        .neighbors_directed(index, Direction::Incoming)
        .filter(|idx| matches!(&graph[*idx], GraphNode::Store(_)))
}

pub fn output_stores(index: NodeIndex, graph: &Graph) -> impl Iterator<Item = NodeIndex> + '_ {
    graph
        .neighbors_directed(index, Direction::Outgoing)
        .filter(|idx| matches!(&graph[*idx], GraphNode::Store(_)))
}
