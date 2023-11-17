use crate::{graph::Graph, node::NodeIndex, ReferenceFamily};

trait BreadthFirstTraversal<'graph>:
    Iterator<Item = <Self::NodeRefFamily as ReferenceFamily>::NodeRef<'graph, Self::Graph>>
{
    type Graph: 'graph + Graph;
    type NodeRefFamily: ReferenceFamily;

    fn new(graph: &'graph Self::Graph, start: NodeIndex<'graph, Self::Graph>) -> Self;
}
