use std::fmt::Debug;

use num_traits::PrimInt;

use crate::edge::{EdgeFamily, EdgeIndex};
use crate::ReferenceFamily;
//use crate::edges::Edges;
use crate::node::NodeIndex;

pub trait Graph: Sized {
    type NodeIdx: PrimInt + Debug;
    type EdgeFamily: EdgeFamily;

    type IngoingEdgesIter<'graph, F>: Iterator<Item = F::EdgeRef<'graph, Self>>
    where
        F: ReferenceFamily,
        Self: 'graph;

    type OutgoingEdgesIter<'graph, F>: Iterator<Item = F::EdgeRef<'graph, Self>>
    where
        F: ReferenceFamily,
        Self: 'graph;

    type NodesIter<'graph, F>: Iterator<Item = F::NodeRef<'graph, Self>>
    where
        F: ReferenceFamily,
        Self: 'graph;

    type EdgesIter<'graph, F>: Iterator<Item = F::EdgeRef<'graph, Self>>
    where
        F: ReferenceFamily,
        Self: 'graph;

    fn order(&self) -> usize;
    fn size(&self) -> usize;

    fn nodes<F>(&self) -> Self::NodesIter<'_, F>
    where
        F: ReferenceFamily;
    fn edges<F>(&self) -> Self::EdgesIter<'_, F>
    where
        F: ReferenceFamily;

    fn get_node<F: ReferenceFamily>(&self, idx: Self::NodeIdx) -> Option<F::NodeRef<'_, Self>>;

    fn indegree(&self, node: NodeIndex<'_, Self>) -> usize;

    fn outdegree(&self, node: NodeIndex<'_, Self>) -> usize;

    fn inedges<'graph, F>(
        &'graph self,
        node: NodeIndex<'graph, Self>,
    ) -> Self::IngoingEdgesIter<'graph, F>
    where
        F: ReferenceFamily;

    fn outedges<'graph, F>(
        &'graph self,
        node: NodeIndex<'graph, Self>,
    ) -> Self::OutgoingEdgesIter<'graph, F>
    where
        F: ReferenceFamily;
}

pub trait WithNodeWeight: Graph {
    type NodeWeight;

    fn node_weight(&self, node: NodeIndex<'_, Self>) -> &'_ Self::NodeWeight;
}

pub trait WithEgdeWeight: Graph {
    type EdgeWeight;

    fn edge_weight(&self, edge: EdgeIndex<'_, Self>) -> &'_ Self::EdgeWeight;
}
