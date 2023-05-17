use super::{Edge, EdgeFamily};
use crate::idx::Index;

use std::hash::Hash;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct DirectedEdge<I: Index> {
//     pub from: I,
//     pub to: I,
// }
// impl<I: Index> Edge for DirectedEdge<I> {
//     type NodeIdx = I;
//     fn new(from: I, to: I) -> Self {
//         Self { from, to }
//     }
// }

pub struct Directed {}
impl EdgeFamily for Directed {
    //     type Edge<NodeIdx> = DirectedEdge<NodeIdx> where NodeIdx: Index;
}

impl<I> PartialEq for Edge<Directed, I>
where
    I: Index,
{
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge
    }
}
impl<I> Eq for Edge<Directed, I> where I: Index {}

impl<I> Hash for Edge<Directed, I>
where
    I: Index + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edge.hash(state);
    }
}

impl<I> Edge<Directed, I>
where
    I: Index,
{
    pub fn from(&self) -> I {
        self.edge.0
    }

    pub fn to(&self) -> I {
        self.edge.1
    }
}
