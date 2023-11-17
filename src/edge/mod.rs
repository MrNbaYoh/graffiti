use std::marker::PhantomData;

mod directed;

mod undirected;

mod family;
pub use family::{Directed, EdgeFamily, Undirected};

mod reference;
pub use reference::{EdgeIndex, EdgeRef, EdgeView};

#[derive(Debug, Clone, Copy)]
pub struct Edge<F, I>
where
    F: EdgeFamily,
    I: Clone + Copy + PartialEq + Eq,
{
    pub(crate) edge: (I, I),
    pub(crate) _phantom: PhantomData<F>,
}

impl<F, I> Edge<F, I>
where
    F: EdgeFamily,
    I: Copy + PartialEq + Eq,
{
    pub fn new(from: I, to: I) -> Self {
        Self {
            edge: (from, to),
            _phantom: PhantomData,
        }
    }
}

impl<F: EdgeFamily, I: Copy + PartialEq + Eq> Edge<F, I> {
    pub fn from(&self) -> I {
        self.edge.0
    }

    pub fn to(&self) -> I {
        self.edge.1
    }
}

impl<F, I> From<Edge<F, I>> for (I, I)
where
    F: EdgeFamily,
    I: Copy + PartialEq + Eq,
{
    fn from(edge: Edge<F, I>) -> Self {
        edge.edge
    }
}

impl<F, I> From<(I, I)> for Edge<F, I>
where
    F: EdgeFamily,
    I: Copy + PartialEq + Eq,
{
    fn from(edge: (I, I)) -> Self {
        Self::new(edge.0, edge.1)
    }
}
