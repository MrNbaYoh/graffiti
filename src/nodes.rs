use crate::graph::{Graph, GraphNodeIdx};

//TODO: add graph lifetime?
pub trait Nodes {
    type Graph: Graph;
    type IndicesIter: Iterator<Item = GraphNodeIdx<Self::Graph>>;

    fn count(&self) -> usize;
    fn indices(&self) -> Self::IndicesIter;
}

pub type GraphNodeValue<G> = <G as WithNodeValues>::NodeValue;
pub trait WithNodeValues: Graph
where
    <Self as Graph>::Nodes: NodesWithValues<Graph = Self>,
{
    type NodeValue;
}

impl<G> WithNodeValues for G
where
    G: Graph,
    G::Nodes: NodesWithValues<Graph = G>,
{
    type NodeValue = <G::Nodes as NodesWithValues>::Value;
}

pub trait NodesWithValues: Nodes {
    type Value;
    type ValuesIter<'graph>: Iterator<Item = (GraphNodeIdx<Self::Graph>, &'graph Self::Value)>
    where
        Self: 'graph,
        Self::Value: 'graph;

    fn value(&self, idx: GraphNodeIdx<Self::Graph>) -> Option<&Self::Value>;
    fn values(&self) -> Self::ValuesIter<'_>;
}
