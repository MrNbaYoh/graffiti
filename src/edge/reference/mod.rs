mod index;
pub use index::EdgeIndex;

mod view;
pub use view::EdgeView;

use crate::{graph::Graph, ReferenceFamily};

pub trait EdgeRef<'graph>:
    Clone
    + Copy
    // + PartialEq
    // + Eq
    + Into<(
        <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>,
        <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>,
    )>
{
    type Graph: 'graph + Graph;
    type NodeRefFamily: ReferenceFamily;

    fn assume_exists(from: <Self::Graph as Graph>::NodeIdx, to: <Self::Graph as Graph>::NodeIdx, graph: &'graph Self::Graph) -> Self;

    fn from(&self) -> <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>;

    fn to(&self) -> <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>;
}
