use sealed::sealed;

use crate::{algorithm::cyclic::CycleDetectionAlgorithm, graph::Graph};

#[sealed]
pub trait CyclicAttribute {
    fn check_cyclic<A: CycleDetectionAlgorithm + ?Sized, G: Graph>(g: &G) -> bool;
}

pub trait WithCyclicAttribute: Graph {
    type Value: CyclicAttribute;
}

pub struct Unknown;
pub struct Acyclic;
pub struct Cyclic;

#[sealed]
impl CyclicAttribute for Unknown {
    #[inline]
    fn check_cyclic<A: CycleDetectionAlgorithm + ?Sized, G: Graph>(g: &G) -> bool {
        A::run_raw(g)
    }
}
#[sealed]
impl CyclicAttribute for Acyclic {
    #[inline]
    fn check_cyclic<A: CycleDetectionAlgorithm + ?Sized, G: Graph>(_: &G) -> bool {
        false
    }
}
#[sealed]
impl CyclicAttribute for Cyclic {
    #[inline]
    fn check_cyclic<A: CycleDetectionAlgorithm + ?Sized, G: Graph>(_: &G) -> bool {
        true
    }
}
