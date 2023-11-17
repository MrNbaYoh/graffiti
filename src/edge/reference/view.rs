use std::marker::PhantomData;

use crate::{
    edge::Edge,
    graph::{Graph, WithEgdeWeight},
    node::{NodeIndex, NodeRef, NodeView},
};

use super::EdgeRef;

#[derive(Debug)]
pub struct EdgeView<'graph, G: 'graph + Graph> {
    pub(crate) edge: Edge<G::EdgeFamily, NodeIndex<'graph, G>>,
    pub(crate) graph: &'graph G,
}

impl<'graph, G: 'graph + Graph> From<EdgeView<'graph, G>>
    for (NodeView<'graph, G>, NodeView<'graph, G>)
{
    fn from(value: EdgeView<'graph, G>) -> Self {
        let (node1, node2): (NodeIndex<_>, NodeIndex<_>) = value.edge.into();
        let view1 = NodeView::assume_exists(node1.index(), value.graph);
        let view2 = NodeView::assume_exists(node2.index(), value.graph);
        (view1, view2)
    }
}

impl<'graph, G: 'graph + Graph> PartialEq for EdgeView<'graph, G>
where
    Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge
    }
}
impl<'graph, G: 'graph + Graph> Eq for EdgeView<'graph, G> where
    Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq
{
}

impl<'graph, G: 'graph + Graph> Clone for EdgeView<'graph, G> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge,
            graph: self.graph,
        }
    }
}
impl<'graph, G: 'graph + Graph> Copy for EdgeView<'graph, G> {}

impl<'graph, G: 'graph + Graph> EdgeRef<'graph> for EdgeView<'graph, G>
// where
//     Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq,
{
    type Graph = G;
    type NodeRefFamily = crate::ref_family::View;

    fn from(&self) -> NodeView<'graph, Self::Graph> {
        NodeView::assume_exists(self.edge.from().index(), self.graph)
    }

    fn to(&self) -> NodeView<'graph, Self::Graph> {
        NodeView::assume_exists(self.edge.to().index(), self.graph)
    }

    fn assume_exists(from: G::NodeIdx, to: G::NodeIdx, graph: &'graph Self::Graph) -> Self {
        let from = NodeIndex::assume_exists(from, graph);
        let to = NodeIndex::assume_exists(to, graph);
        let edge = Edge {
            edge: (from, to),
            _phantom: PhantomData,
        };
        Self { edge, graph }
    }
}

impl<'graph, G: 'graph + Graph> EdgeView<'graph, G> {
    pub fn weight(&self) -> &'graph G::EdgeWeight
    where
        G: WithEgdeWeight,
    {
        self.graph.edge_weight((*self).into())
    }
}
