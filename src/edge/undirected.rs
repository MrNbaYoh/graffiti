use super::{family::Undirected, Edge};
use std::hash::Hash;

impl<I: PartialEq + Eq + Copy> PartialEq for Edge<Undirected, I> {
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge || (self.edge == (other.edge.1, other.edge.0))
    }
}
impl<I: Eq + Copy> Eq for Edge<Undirected, I> {}

impl<I> Hash for Edge<Undirected, I>
where
    I: Copy + Hash + PartialOrd + Ord,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let edge = match self.edge.0.cmp(&self.edge.1) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => self.edge,
            std::cmp::Ordering::Greater => (self.edge.1, self.edge.0),
        };
        edge.hash(state);
    }
}
