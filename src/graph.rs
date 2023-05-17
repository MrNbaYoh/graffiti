use crate::edge::{Edge, EdgeFamily};
use crate::edges::Edges;
use crate::idx::Index;
use crate::nodes::Nodes;
use crate::predecessors::Predecessors;
use crate::successors::Successors;

pub trait Graph: Sized {
    //TODO: add lifetime on indices?
    type NodeIdx: Index;
    type Nodes: Nodes<Self>;

    type EdgeFamily: EdgeFamily;
    type Edges: Edges<Self>;

    //type NeighborIter: Iterator<Item = Self::NodeIdx>;
    //TODO: lifetime on iters?
    type Successors: Successors<Graph = Self>;
    type Predecessors: Predecessors<Graph = Self>;

    fn node_count(&self) -> usize;
    fn nodes(&self) -> Self::Nodes;

    fn edges(&self) -> Self::Edges;

    // fn neighbors(&self, node: Self::NodeIdx) -> Option<Self::NeighborIter>;
    fn successors(&self, node: Self::NodeIdx) -> Option<Self::Successors>;
    fn predecessors(&self, node: Self::NodeIdx) -> Option<Self::Predecessors>;
}

pub type GraphNodeIdx<G> = <G as Graph>::NodeIdx;
pub type GraphEdge<G> = Edge<<G as Graph>::EdgeFamily, <G as Graph>::NodeIdx>;
