use crate::graph::{Graph, GraphEdge};

//TODO: add graph lifetime?
pub trait Edges {
    type Graph: Graph;
    type Iter: Iterator<Item = GraphEdge<Self::Graph>>;

    fn count(&self) -> usize;
    fn iter(&self) -> Self::Iter;
}

pub trait EdgesWithValues: Edges {
    type Value;
    type ValuesIter<'graph>: Iterator<Item = (GraphEdge<Self::Graph>, &'graph Self::Value)>
    where
        Self: 'graph,
        Self::Value: 'graph;

    fn value(&self, edge: GraphEdge<Self::Graph>) -> Option<&Self::Value>;
    fn values(&self) -> Self::ValuesIter<'_>;
}

pub type GraphEdgeValue<G> = <G as WithEdgeValues>::EdgeValue;
pub trait WithEdgeValues: Graph
where
    Self::Edges: EdgesWithValues<Graph = Self>,
{
    type EdgeValue;
}

impl<G> WithEdgeValues for G
where
    G: Graph,
    G::Edges: EdgesWithValues<Graph = G>,
{
    type EdgeValue = <G::Edges as EdgesWithValues>::Value;
}
