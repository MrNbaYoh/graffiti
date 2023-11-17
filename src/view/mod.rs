mod transpose;
use std::marker::PhantomData;

pub use transpose::*;

use crate::{
    graph::Graph,
    node::{NodeIndex, NodeView},
};

pub trait ViewOfNodes<'graph>: Graph<NodeIdx = <Self::Graph as Graph>::NodeIdx> {
    type Graph: 'graph + Graph;

    //fn index_of(&self, node: NodeIndex<'graph, Self::Graph>) -> NodeIndex<'_, Self>;
}

impl<'graph, G: 'graph + Graph> NodeIndex<'graph, G> {
    pub fn same<'v, V: ViewOfNodes<'v, Graph = G>>(self) -> NodeIndex<'v, V> {
        NodeIndex {
            index: self.index,
            _phantom: PhantomData,
        }
    }
}

impl<'graph, G: 'graph + Graph> NodeView<'graph, G> {
    pub fn same<'v, 'g, V: ViewOfNodes<'g, Graph = G>>(self, v: &'v V) -> NodeView<'v, V> {
        NodeView {
            graph: v,
            index: self.index,
        }
    }
}
