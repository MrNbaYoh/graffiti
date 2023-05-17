use crate::idx::Index;
use std::marker::PhantomData;

mod directed;
pub use directed::Directed;

mod undirected;
pub use undirected::Undirected;

mod family;
pub use family::EdgeFamily;

#[derive(Debug, Clone, Copy)]
pub struct Edge<F, I>
where
    F: EdgeFamily,
    I: Index,
{
    pub(crate) edge: (I, I),
    pub(crate) _phantom: PhantomData<F>,
}

impl<F, I> Edge<F, I>
where
    F: EdgeFamily,
    I: Index,
{
    pub fn new(from: I, to: I) -> Self {
        Self {
            edge: (from, to),
            _phantom: PhantomData,
        }
    }
}

impl<F, I> From<Edge<F, I>> for (I, I)
where
    F: EdgeFamily,
    I: Index,
{
    fn from(edge: Edge<F, I>) -> Self {
        edge.edge
    }
}
