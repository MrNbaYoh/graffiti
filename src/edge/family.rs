pub trait EdgeFamily: sealed::Sealed {
    // type Edge<NodeIdx>: Edge<NodeIdx = NodeIdx>
    // where
    //     NodeIdx: Index;
}

// pub trait Edge: sealed::Sealed {
//     type NodeIdx: Index;
//     fn new(from: Self::NodeIdx, to: Self::NodeIdx) -> Self;
// }

mod sealed {
    // use crate::idx::Index;
    pub trait Sealed {}
    impl Sealed for crate::edge::Directed {}
    impl Sealed for crate::edge::Undirected {}
    // impl<I: Index> Sealed for super::DirectedEdge<I> {}
    // impl<I: Index> Sealed for super::UndirectedEdge<I> {}
}
