use crate::{
    direction::Direction,
    edge::{Directed, Undirected},
    //graph::GetNodeNeighbors,
    ReferenceFamily,
};
use std::marker::PhantomData;

use crate::graph::{Graph, WithNodeWeight};

use super::{NodeIndex, NodeRef};

pub struct NodeView<'graph, G: Graph + ?Sized> {
    pub(crate) index: G::NodeIdx,
    pub(crate) graph: &'graph G,
}

impl<G: Graph> Clone for NodeView<'_, G> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            graph: self.graph,
        }
    }
}

impl<G: Graph> Copy for NodeView<'_, G> {}

impl<G: Graph> PartialEq for NodeView<'_, G> {
    fn eq(&self, other: &Self) -> bool {
        //FIXME: compare graph references?
        self.index == other.index
    }
}

impl<G: Graph> Eq for NodeView<'_, G> {}

impl<'graph, G: Graph> NodeRef<'graph> for NodeView<'graph, G> {
    type Graph = G;

    fn assume_exists(index: G::NodeIdx, graph: &'graph G) -> Self {
        Self { index, graph }
    }

    fn index(&self) -> G::NodeIdx {
        self.index
    }
}

impl<'graph, G: Graph> From<NodeView<'graph, G>> for NodeIndex<'graph, G> {
    fn from(value: NodeView<'graph, G>) -> Self {
        NodeIndex {
            index: value.index(),
            _phantom: PhantomData,
        }
    }
}

impl<'graph, G: Graph> NodeView<'graph, G> {
    pub fn weight(&self) -> &'graph G::NodeWeight
    where
        G: WithNodeWeight,
    {
        self.graph.node_weight((*self).into())
    }

    // pub fn degree(&self) -> usize
    // where
    //     G: Graph<EdgeFamily = Undirected>,
    // {
    //     self.graph.degree((*self).into())
    // }

    pub fn indegree(&self) -> usize
// where
    //     G: Graph<EdgeFamily = Directed>,
    {
        self.graph.indegree((*self).into())
    }

    pub fn outdegree(&self) -> usize
// where
    //     G: Graph<EdgeFamily = Directed>,
    {
        self.graph.outdegree((*self).into())
    }

    pub fn inedges<F>(&self) -> G::IngoingEdgesIter<'graph, F>
    where
        F: ReferenceFamily,
    {
        self.graph.inedges((*self).into())
    }

    pub fn outedges<F>(&self) -> G::OutgoingEdgesIter<'graph, F>
    where
        F: ReferenceFamily,
    {
        self.graph.outedges((*self).into())
    }
}
