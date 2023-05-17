use crate::graph::{Graph, GraphEdge};

//TODO: add graph lifetime?
pub trait Edges<G: Graph> {
    type Iter: Iterator<Item = GraphEdge<G>>;

    fn count(&self) -> usize;
    fn iter(&self) -> Self::Iter;
}

pub trait EdgesWithValues<G: Graph>: Edges<G> {
    type Value;
    type ValuesIter<'graph>: Iterator<Item = (GraphEdge<G>, &'graph Self::Value)>
    where
        Self: 'graph,
        Self::Value: 'graph;

    fn value(&self, edge: GraphEdge<G>) -> Option<&Self::Value>;
    fn values(&self) -> Self::ValuesIter<'_>;
}

pub type GraphEdgeValue<G> = <G as WithEdgeValues>::EdgeValue;
pub trait WithEdgeValues: Graph
where
    Self::Edges: EdgesWithValues<Self>,
{
    type EdgeValue;
}

impl<G> WithEdgeValues for G
where
    G: Graph,
    G::Edges: EdgesWithValues<G>,
{
    type EdgeValue = <G::Edges as EdgesWithValues<G>>::Value;
}
