use crate::graph::{Graph, GraphEdge, GraphNodeIdx};
use std::marker::PhantomData;

pub trait Successors {
    type Graph: Graph;
    type Iter: Iterator<Item = GraphNodeIdx<Self::Graph>>;

    fn from(&self) -> GraphNodeIdx<Self::Graph>;
    //TODO: require impl iterator instead of building new iter?
    fn iter(&self) -> Self::Iter;

    //TODO: "into" instead?
    fn edges(&self) -> SuccessorsEdgesIter<Self::Graph, Self::Iter> {
        SuccessorsEdgesIter {
            from: self.from(),
            iter: self.iter(),
            _phantom: PhantomData,
        }
    }
}

pub struct SuccessorsEdgesIter<G: Graph, I: Iterator<Item = G::NodeIdx>> {
    from: I::Item,
    iter: I,
    _phantom: PhantomData<G>,
}

impl<G: Graph, I: Iterator<Item = G::NodeIdx>> Iterator for SuccessorsEdgesIter<G, I> {
    type Item = GraphEdge<G>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|node| GraphEdge::<G>::new(self.from, node))
    }
}
