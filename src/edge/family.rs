use sealed::sealed;

#[sealed]
pub trait EdgeFamily: Clone + Copy {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Directed;
#[sealed]
impl EdgeFamily for Directed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Undirected;
#[sealed]
impl EdgeFamily for Undirected {}
