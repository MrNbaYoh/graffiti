use super::{family::Directed, Edge};
use std::hash::Hash;

impl<I: PartialEq + Eq + Copy> PartialEq for Edge<Directed, I> {
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge
    }
}
impl<I: PartialEq + Eq + Copy> Eq for Edge<Directed, I> {}

impl<I: PartialEq + Eq + Copy + Hash> Hash for Edge<Directed, I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edge.hash(state);
    }
}
