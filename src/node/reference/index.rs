use std::marker::PhantomData;

use crate::graph::Graph;

use super::NodeRef;

#[derive(Debug)]
pub struct NodeIndex<'graph, G: Graph + ?Sized> {
    pub(crate) index: G::NodeIdx,
    pub(crate) _phantom: PhantomData<&'graph G>,
}

impl<G: Graph> Clone for NodeIndex<'_, G> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _phantom: PhantomData,
        }
    }
}

impl<G: Graph> Copy for NodeIndex<'_, G> {}

impl<G: Graph> PartialEq for NodeIndex<'_, G> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<G: Graph> Eq for NodeIndex<'_, G> {}

impl<'graph, G: Graph> NodeRef<'graph> for NodeIndex<'graph, G> {
    type Graph = G;

    fn assume_exists(index: G::NodeIdx, _graph: &'graph G) -> Self {
        Self {
            index,
            _phantom: PhantomData,
        }
    }

    fn index(&self) -> G::NodeIdx {
        self.index
    }
}
