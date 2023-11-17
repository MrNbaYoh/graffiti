use crate::{
    attribute::cyclic::{Acyclic, Cyclic, CyclicAttribute, Unknown, WithCyclicAttribute},
    graph::Graph,
};

pub enum CycleDetectionResult<A, C>
where
    A: Graph + WithCyclicAttribute<Value = Acyclic>,
    C: Graph + WithCyclicAttribute<Value = Cyclic>,
{
    Acyclic(A),
    Cyclic(C),
}

pub trait CycleDetectionAlgorithm {
    fn run_raw<G: Graph>(g: &G) -> bool;

    fn is_cyclic<G: Graph + WithCyclicAttribute>(g: &G) -> bool {
        <<G as WithCyclicAttribute>::Value as CyclicAttribute>::check_cyclic::<Self, G>(g)
    }

    fn check_into<G, A, C>(g: G) -> CycleDetectionResult<A, C>
    where
        G: Graph + WithCyclicAttribute,
        A: Graph + WithCyclicAttribute<Value = Acyclic>,
        C: Graph + WithCyclicAttribute<Value = Cyclic>;
}
