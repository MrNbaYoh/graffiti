use std::marker::PhantomData;

use crate::{
    edge::{Directed, EdgeFamily, EdgeRef},
    graph::Graph,
    node::{NodeIndex, NodeRef},
    Index, ReferenceFamily,
};

use super::ViewOfNodes;

pub struct TransposeView<'g, G: 'g + Graph<EdgeFamily = Directed>> {
    graph: &'g G,
}

impl<'g, G: 'g + Graph<EdgeFamily = Directed>> TransposeView<'g, G> {
    pub fn of(graph: &'g G) -> Self {
        TransposeView { graph }
    }
}

pub struct TransposedNodeIter<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> {
    graph: &'t TransposeView<'g, G>,
    iter: G::NodesIter<'g, Index>,
    _phantom: PhantomData<F>,
}

impl<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> Iterator
    for TransposedNodeIter<'t, 'g, G, F>
{
    type Item = F::NodeRef<'t, TransposeView<'g, G>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| {
            let index = node.index();
            <Self::Item as NodeRef>::assume_exists(index, self.graph)
        })
    }
}

pub struct TransposedEdgeIter<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> {
    graph: &'t TransposeView<'g, G>,
    iter: G::EdgesIter<'g, Index>,
    _phantom: PhantomData<F>,
}

impl<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> Iterator
    for TransposedEdgeIter<'t, 'g, G, F>
{
    type Item = F::EdgeRef<'t, TransposeView<'g, G>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| {
            let (from, to) = edge.into();
            <Self::Item as EdgeRef>::assume_exists(to.index(), from.index(), self.graph)
        })
    }
}

pub struct TransposedIngoingEdgeIter<
    't,
    'g,
    G: 'g + Graph<EdgeFamily = Directed>,
    F: ReferenceFamily,
> {
    graph: &'t TransposeView<'g, G>,
    iter: G::OutgoingEdgesIter<'g, Index>,
    _phantom: PhantomData<F>,
}

impl<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> Iterator
    for TransposedIngoingEdgeIter<'t, 'g, G, F>
{
    type Item = F::EdgeRef<'t, TransposeView<'g, G>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| {
            let (from, to) = edge.into();
            <Self::Item as EdgeRef>::assume_exists(to.index(), from.index(), self.graph)
        })
    }
}

pub struct TransposedOutgoingEdgeIter<
    't,
    'g,
    G: 'g + Graph<EdgeFamily = Directed>,
    F: ReferenceFamily,
> {
    graph: &'t TransposeView<'g, G>,
    iter: G::IngoingEdgesIter<'g, Index>,
    _phantom: PhantomData<F>,
}

impl<'t, 'g, G: 'g + Graph<EdgeFamily = Directed>, F: ReferenceFamily> Iterator
    for TransposedOutgoingEdgeIter<'t, 'g, G, F>
{
    type Item = F::EdgeRef<'t, TransposeView<'g, G>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| {
            let (from, to) = edge.into();
            <Self::Item as EdgeRef>::assume_exists(to.index(), from.index(), self.graph)
        })
    }
}

impl<'g, G: 'g + Graph<EdgeFamily = Directed>> Graph for TransposeView<'g, G> {
    type NodeIdx = G::NodeIdx;

    type EdgeFamily = Directed;

    type NodesIter<'graph, F> = TransposedNodeIter<'graph, 'g, G, F>
    where
        F: crate::ReferenceFamily,
        Self: 'graph;

    type EdgesIter<'graph, F> = TransposedEdgeIter<'graph, 'g, G, F>
    where
        F: crate::ReferenceFamily,
        Self: 'graph;

    type IngoingEdgesIter<'graph, F> = TransposedIngoingEdgeIter<'graph, 'g, G, F>
    where
        F: crate::ReferenceFamily,
        Self: 'graph;

    type OutgoingEdgesIter<'graph, F> = TransposedOutgoingEdgeIter<'graph, 'g, G, F>
    where
        F: crate::ReferenceFamily,
        Self: 'graph;

    fn order(&self) -> usize {
        self.graph.order()
    }

    fn size(&self) -> usize {
        self.graph.size()
    }

    fn nodes<F>(&self) -> Self::NodesIter<'_, F>
    where
        F: crate::ReferenceFamily,
    {
        TransposedNodeIter {
            graph: self,
            iter: self.graph.nodes(),
            _phantom: PhantomData,
        }
    }

    fn edges<F>(&self) -> Self::EdgesIter<'_, F>
    where
        F: crate::ReferenceFamily,
    {
        todo!()
    }

    fn get_node<F: crate::ReferenceFamily>(
        &self,
        idx: Self::NodeIdx,
    ) -> Option<F::NodeRef<'_, Self>> {
        if self.graph.get_node::<Index>(idx).is_some() {
            let node = <F::NodeRef<'_, Self> as NodeRef>::assume_exists(idx, self);
            Some(node)
        } else {
            None
        }
    }

    fn indegree(&self, node: crate::node::NodeIndex<'_, Self>) -> usize {
        let node = NodeIndex::assume_exists(node.index(), self.graph);
        self.graph.outdegree(node)
    }

    fn outdegree(&self, node: crate::node::NodeIndex<'_, Self>) -> usize {
        let node = NodeIndex::assume_exists(node.index(), self.graph);
        self.graph.indegree(node)
    }

    fn inedges<'graph, F>(
        &'graph self,
        node: crate::node::NodeIndex<'graph, Self>,
    ) -> Self::IngoingEdgesIter<'graph, F>
    where
        F: crate::ReferenceFamily,
    {
        let node = <NodeIndex<G> as NodeRef>::assume_exists(node.index(), self.graph);
        TransposedIngoingEdgeIter {
            graph: self,
            iter: self.graph.outedges(node),
            _phantom: PhantomData,
        }
    }

    fn outedges<'graph, F>(
        &'graph self,
        node: crate::node::NodeIndex<'graph, Self>,
    ) -> Self::OutgoingEdgesIter<'graph, F>
    where
        F: crate::ReferenceFamily,
    {
        let node = <NodeIndex<G> as NodeRef>::assume_exists(node.index(), self.graph);
        TransposedOutgoingEdgeIter {
            graph: self,
            iter: self.graph.inedges(node),
            _phantom: PhantomData,
        }
    }
}

impl<'g, G: 'g + Graph<EdgeFamily = Directed>> ViewOfNodes<'g> for TransposeView<'g, G> {
    type Graph = G;
}
