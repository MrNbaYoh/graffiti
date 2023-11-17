use sealed::sealed;

use crate::{
    edge::{EdgeIndex, EdgeRef, EdgeView},
    graph::Graph,
    node::{NodeIndex, NodeRef, NodeView},
};

#[sealed]
pub trait ReferenceFamily {
    type EdgeRef<'graph, G>: 'graph + EdgeRef<'graph, Graph = G>
    where
        G: 'graph + Graph;
    type NodeRef<'graph, G>: 'graph + NodeRef<'graph, Graph = G>
    where
        G: 'graph + Graph;
}

#[sealed]
impl ReferenceFamily for Index {
    type NodeRef<'graph, G> = NodeIndex<'graph, G> where G: 'graph + Graph;
    type EdgeRef<'graph, G> = EdgeIndex<'graph, G> where G: 'graph + Graph;
}

#[sealed]
impl ReferenceFamily for View {
    type EdgeRef<'graph, G> = EdgeView<'graph, G> where G: 'graph + Graph;
    type NodeRef<'graph, G> = NodeView<'graph, G> where G: 'graph + Graph;
}

pub struct Index {}
pub struct View {}
