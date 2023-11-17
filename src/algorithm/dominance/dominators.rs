use crate::{edge::Directed, graph::Graph, node::NodeIndex};

pub trait DominatorTree<'graph> {
    type Graph: 'graph + Graph<EdgeFamily = Directed>;

    type DominatorsIter<'this>: Iterator<Item = NodeIndex<'graph, Self::Graph>>
    where
        Self: 'this;

    type StrictDominatorsIter<'this>: Iterator<Item = NodeIndex<'graph, Self::Graph>>
    where
        Self: 'this;

    type DominatedByIter<'this>: Iterator<Item = NodeIndex<'graph, Self::Graph>>
    where
        Self: 'this;

    fn from_root(graph: &'graph Self::Graph, node: NodeIndex<'graph, Self::Graph>) -> Self;

    fn immediate_dominator(
        &self,
        node: NodeIndex<'graph, Self::Graph>,
    ) -> Option<NodeIndex<'graph, Self::Graph>>;
    fn immediately_dominated_by(
        &self,
        node: NodeIndex<'graph, Self::Graph>,
    ) -> Self::DominatedByIter<'_>;
    fn dominators(&self, node: NodeIndex<'graph, Self::Graph>) -> Self::DominatorsIter<'_>;
    fn strict_dominators(
        &self,
        node: NodeIndex<'graph, Self::Graph>,
    ) -> Self::StrictDominatorsIter<'_>;
}
