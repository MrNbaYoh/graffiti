use crate::graph::{Graph, GraphEdge, GraphNodeIdx};
use std::marker::PhantomData;

pub trait Predecessors {
    type Graph: Graph;
    type Iter: Iterator<Item = GraphNodeIdx<Self::Graph>>;

    fn to(&self) -> GraphNodeIdx<Self::Graph>;
    //TODO: require impl iterator instead of building new iter?
    fn iter(&self) -> Self::Iter;

    //TODO: "into" instead?
    fn edges(&self) -> PredecessorsEdgesIter<Self::Graph, Self::Iter> {
        PredecessorsEdgesIter {
            to: self.to(),
            iter: self.iter(),
            _phantom: PhantomData,
        }
    }
}

pub struct PredecessorsEdgesIter<G: Graph, I: Iterator<Item = G::NodeIdx>> {
    to: I::Item,
    iter: I,
    _phantom: PhantomData<G>,
}

impl<G: Graph, I: Iterator<Item = G::NodeIdx>> Iterator for PredecessorsEdgesIter<G, I> {
    type Item = GraphEdge<G>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|node| GraphEdge::<G>::new(node, self.to))
    }
}
