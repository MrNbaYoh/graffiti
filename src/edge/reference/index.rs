use std::marker::PhantomData;

use crate::{
    edge::Edge,
    graph::Graph,
    node::{NodeIndex, NodeRef},
};

use super::{EdgeRef, EdgeView};

#[derive(Debug)]
pub struct EdgeIndex<'graph, G: 'graph + Graph> {
    pub(crate) edge: Edge<G::EdgeFamily, NodeIndex<'graph, G>>,
}

impl<'graph, G: 'graph + Graph> From<EdgeIndex<'graph, G>>
    for (NodeIndex<'graph, G>, NodeIndex<'graph, G>)
{
    fn from(value: EdgeIndex<'graph, G>) -> Self {
        value.edge.into()
    }
}

impl<'graph, G: 'graph + Graph> PartialEq for EdgeIndex<'graph, G>
where
    Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge
    }
}
impl<'graph, G: 'graph + Graph> Eq for EdgeIndex<'graph, G> where
    Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq
{
}

impl<'graph, G: 'graph + Graph> Clone for EdgeIndex<'graph, G> {
    fn clone(&self) -> Self {
        Self { edge: self.edge }
    }
}
impl<'graph, G: 'graph + Graph> Copy for EdgeIndex<'graph, G> {}

impl<'graph, G: 'graph + Graph> EdgeRef<'graph> for EdgeIndex<'graph, G>
// where
//     Edge<G::EdgeFamily, NodeIndex<'graph, G>>: PartialEq,
{
    type Graph = G;
    type NodeRefFamily = crate::ref_family::Index;

    fn from(&self) -> NodeIndex<'graph, Self::Graph> {
        self.edge.from()
    }

    fn to(&self) -> NodeIndex<'graph, Self::Graph> {
        self.edge.to()
    }

    fn assume_exists(from: G::NodeIdx, to: G::NodeIdx, graph: &'graph Self::Graph) -> Self {
        let from = NodeIndex::assume_exists(from, graph);
        let to = NodeIndex::assume_exists(to, graph);
        let edge = Edge {
            edge: (from, to),
            _phantom: PhantomData,
        };
        Self { edge }
    }
}

impl<'graph, G: 'graph + Graph> From<EdgeView<'graph, G>> for EdgeIndex<'graph, G> {
    fn from(value: EdgeView<'graph, G>) -> Self {
        let (node1, node2) = value.edge.into();
        let edge = Edge {
            edge: (node1, node2),
            _phantom: PhantomData,
        };
        Self { edge }
    }
}
