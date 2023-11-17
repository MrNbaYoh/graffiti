use crate::attribute::cyclic::{Acyclic, WithCyclicAttribute};
use crate::edge::Directed;
use crate::node::NodeIndex;
use crate::{graph::Graph, ReferenceFamily};

trait TopologicalTraversal<'graph>:
    Iterator<Item = <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>>
{
    type Graph: 'graph + Graph<EdgeFamily = Directed> + WithCyclicAttribute<Value = Acyclic>;
    type NodeRefFamily: ReferenceFamily;

    fn new(graph: &'graph Self::Graph, start: NodeIndex<'graph, Self::Graph>) -> Self;
}
