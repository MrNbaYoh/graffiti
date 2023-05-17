use crate::graph::{Graph, GraphNodeIdx};

//TODO: add graph lifetime?
pub trait Nodes<G: Graph> {
    type IndicesIter: Iterator<Item = GraphNodeIdx<G>>;

    fn count(&self) -> usize;
    fn indices(&self) -> Self::IndicesIter;
}

pub type GraphNodeValue<G> = <G as WithNodeValues>::NodeValue;
pub trait WithNodeValues: Graph
where
    <Self as Graph>::Nodes: NodesWithValues<Self>,
{
    type NodeValue;
}

impl<G> WithNodeValues for G
where
    G: Graph,
    G::Nodes: NodesWithValues<G>,
{
    type NodeValue = <G::Nodes as NodesWithValues<G>>::Value;
}

pub trait NodesWithValues<G: Graph>: Nodes<G> {
    type Value;
    type ValuesIter<'graph>: Iterator<Item = (GraphNodeIdx<G>, &'graph Self::Value)>
    where
        Self: 'graph,
        Self::Value: 'graph;

    fn value(&self, idx: G::NodeIdx) -> Option<&Self::Value>;
    fn values(&self) -> Self::ValuesIter<'_>;
}
