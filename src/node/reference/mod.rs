mod index;
pub use index::NodeIndex;

mod view;
pub use view::NodeView;

use crate::graph::Graph;

pub trait NodeRef<'graph>: Clone + Copy + PartialEq + Eq {
    type Graph: Graph;

    fn assume_exists(index: <Self::Graph as Graph>::NodeIdx, graph: &'graph Self::Graph) -> Self;
    fn index(&self) -> <Self::Graph as Graph>::NodeIdx;
}
