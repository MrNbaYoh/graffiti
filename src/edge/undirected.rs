use super::{Edge, EdgeFamily};
use crate::idx::Index;

use std::{hash::Hash, marker::PhantomData};

// #[derive(Debug, Clone, Copy)]
// pub struct UndirectedEdge<I: Index> {
//     pub first: I,
//     pub second: I,
// }
// impl<I: Index> Edge for UndirectedEdge<I> {
//     type NodeIdx = I;
//     fn new(from: I, to: I) -> Self {
//         Self {
//             first: from,
//             second: to,
//         }
//     }
// }

// impl<I: Index> PartialEq for UndirectedEdge<I> {
//     fn eq(&self, other: &Self) -> bool {
//         (self.first == other.first && self.second == other.second)
//             || (self.first == other.second && self.second == other.first)
//     }
// }
// impl<I: Index> Eq for UndirectedEdge<I> {}

// impl<I> Hash for UndirectedEdge<I>
// where
//     I: Index + PartialOrd + Ord + Hash,
// {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         let (min, max) = match self.first.cmp(&self.second) {
//             std::cmp::Ordering::Less | std::cmp::Ordering::Equal => (self.first, self.second),
//             std::cmp::Ordering::Greater => (self.second, self.first),
//         };
//         min.hash(state);
//         max.hash(state);
//     }
// }

pub struct Undirected {}
impl EdgeFamily for Undirected {
    // type Edge<NodeIdx> = UndirectedEdge<NodeIdx> where NodeIdx: Index;
}

impl<I> PartialEq for Edge<Undirected, I>
where
    I: Index,
{
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge || (self.edge == (other.edge.1, other.edge.0))
    }
}
impl<I> Eq for Edge<Undirected, I> where I: Index {}

impl<I> Hash for Edge<Undirected, I>
where
    I: Index + Hash + PartialOrd + Ord,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let edge = match self.edge.0.cmp(&self.edge.1) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => self.edge,
            std::cmp::Ordering::Greater => (self.edge.1, self.edge.0),
        };
        edge.hash(state);
    }
}

impl<I> From<(I, I)> for Edge<Undirected, I>
where
    I: Index,
{
    fn from(edge: (I, I)) -> Self {
        Self {
            edge,
            _phantom: PhantomData,
        }
    }
}
