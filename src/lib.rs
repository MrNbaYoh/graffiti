pub mod ref_family;
pub use ref_family::{Index, ReferenceFamily, View};
pub mod algorithm;
pub mod attribute;
pub mod direction;
pub mod edge;
pub mod graph;
pub mod node;
pub mod view;

#[cfg(test)]
mod test {
    use std::{iter::successors, marker::PhantomData};

    use crate::{
        edge::Directed,
        edge::EdgeRef,
        graph::Graph,
        node::{NodeRef, NodeView},
        view::TransposeView,
        Index, ReferenceFamily, View,
    };

    struct GraphTest {
        successors: Vec<Vec<u32>>,
    }

    struct GraphTestNodeIter<'graph, F: ReferenceFamily> {
        graph: &'graph GraphTest,
        cur_node: usize,
        _phantom: PhantomData<F>,
    }

    impl<'graph, F: ReferenceFamily> Iterator for GraphTestNodeIter<'graph, F> {
        type Item = F::NodeRef<'graph, GraphTest>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur_node < self.graph.successors.len() {
                let res = <Self::Item as NodeRef>::assume_exists(self.cur_node as u32, self.graph);
                self.cur_node += 1;
                Some(res)
            } else {
                None
            }
        }
    }

    struct GraphTestEdgeIter<'graph, F: ReferenceFamily> {
        graph: &'graph GraphTest,
        cur_node: usize,
        cur_succ: usize,
        _phantom: PhantomData<F>,
    }

    impl<'graph, F: ReferenceFamily> Iterator for GraphTestEdgeIter<'graph, F> {
        type Item = F::EdgeRef<'graph, GraphTest>;

        fn next(&mut self) -> Option<Self::Item> {
            while self.cur_node < self.graph.successors.len()
                && self.cur_succ >= self.graph.successors[self.cur_node].len()
            {
                self.cur_succ = 0;
                self.cur_node += 1;
            }

            if self.cur_node >= self.graph.successors.len() {
                return None;
            } else {
                let res = <Self::Item as EdgeRef>::assume_exists(
                    self.cur_node as u32,
                    self.graph.successors[self.cur_node][self.cur_succ],
                    self.graph,
                );
                self.cur_succ += 1;
                Some(res)
            }
        }
    }

    struct GraphTestOutgoingEdgeIter<'graph, F: ReferenceFamily> {
        graph: &'graph GraphTest,
        node_idx: usize,
        cur_succ: usize,
        _phantom: PhantomData<F>,
    }

    impl<'graph, F: ReferenceFamily> Iterator for GraphTestOutgoingEdgeIter<'graph, F> {
        type Item = F::EdgeRef<'graph, GraphTest>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur_succ < self.graph.successors[self.node_idx].len() {
                let res = <Self::Item as EdgeRef>::assume_exists(
                    self.node_idx as u32,
                    self.graph.successors[self.node_idx][self.cur_succ],
                    self.graph,
                );
                self.cur_succ += 1;
                Some(res)
            } else {
                None
            }
        }
    }

    struct GraphTestIngoingEdgeIter<'graph, F: ReferenceFamily> {
        graph: &'graph GraphTest,
        node_idx: usize,
        cur_node: usize,
        _phantom: PhantomData<F>,
    }

    impl<'graph, F: ReferenceFamily> Iterator for GraphTestIngoingEdgeIter<'graph, F> {
        type Item = F::EdgeRef<'graph, GraphTest>;

        fn next(&mut self) -> Option<Self::Item> {
            while self.cur_node < self.graph.successors.len() {
                let cur_node = self.cur_node;
                self.cur_node += 1;

                if self.graph.successors[cur_node].contains(&(self.node_idx as u32)) {
                    let res = <Self::Item as EdgeRef>::assume_exists(
                        cur_node as u32,
                        self.node_idx as u32,
                        self.graph,
                    );
                    return Some(res);
                }
            }
            None
        }
    }

    impl Graph for GraphTest {
        type NodeIdx = u32;

        type EdgeFamily = Directed;

        type IngoingEdgesIter<'graph, F> = GraphTestIngoingEdgeIter<'graph, F>
        where
            F: ReferenceFamily,
            Self: 'graph;

        type OutgoingEdgesIter<'graph, F> = GraphTestOutgoingEdgeIter<'graph, F>
        where
            F: ReferenceFamily,
            Self: 'graph;

        type NodesIter<'graph, F> = GraphTestNodeIter<'graph, F>
        where
            F: crate::ReferenceFamily,
            Self: 'graph;

        type EdgesIter<'graph, F> = GraphTestEdgeIter<'graph, F>
        where
            F: crate::ReferenceFamily,
            Self: 'graph;

        fn order(&self) -> usize {
            self.successors.len()
        }

        fn size(&self) -> usize {
            self.successors.iter().map(|list| list.len()).sum()
        }

        fn nodes<F>(&self) -> Self::NodesIter<'_, F>
        where
            F: crate::ReferenceFamily,
        {
            GraphTestNodeIter {
                graph: self,
                cur_node: 0,
                _phantom: PhantomData,
            }
        }

        fn edges<F>(&self) -> Self::EdgesIter<'_, F>
        where
            F: crate::ReferenceFamily,
        {
            GraphTestEdgeIter {
                graph: self,
                cur_node: 0,
                cur_succ: 0,
                _phantom: PhantomData,
            }
        }

        fn get_node<F: crate::ReferenceFamily>(
            &self,
            idx: Self::NodeIdx,
        ) -> Option<F::NodeRef<'_, Self>> {
            if (idx as usize) < self.successors.len() {
                Some(<F::NodeRef<'_, GraphTest> as NodeRef>::assume_exists(
                    idx, self,
                ))
            } else {
                None
            }
        }

        fn indegree(&self, node: crate::node::NodeIndex<'_, Self>) -> usize
        where
            Self: Graph<EdgeFamily = crate::edge::Directed>,
        {
            return self
                .successors
                .iter()
                .filter(|list| list.contains(&node.index()))
                .count();
        }

        fn outdegree(&self, node: crate::node::NodeIndex<'_, Self>) -> usize
        where
            Self: Graph<EdgeFamily = crate::edge::Directed>,
        {
            self.successors[node.index() as usize].len()
        }

        fn inedges<'graph, F>(
            &'graph self,
            node: crate::node::NodeIndex<'graph, Self>,
        ) -> Self::IngoingEdgesIter<'graph, F>
        where
            F: ReferenceFamily,
        {
            GraphTestIngoingEdgeIter {
                graph: self,
                node_idx: node.index() as usize,
                cur_node: 0,
                _phantom: PhantomData,
            }
        }

        fn outedges<'graph, F>(
            &'graph self,
            node: crate::node::NodeIndex<'graph, Self>,
        ) -> Self::OutgoingEdgesIter<'graph, F>
        where
            F: ReferenceFamily,
        {
            GraphTestOutgoingEdgeIter {
                graph: self,
                node_idx: node.index() as usize,
                cur_succ: 0,
                _phantom: PhantomData,
            }
        }
    }

    #[test]
    fn test() {
        let g = GraphTest {
            successors: vec![vec![1, 2], vec![], vec![0]],
        };

        let node_1 = g.get_node::<Index>(1).unwrap();

        let node_0 = g.get_node::<View>(0).unwrap();
        println!("OUT OF 0");
        node_0
            .outedges::<Index>()
            .for_each(|edge| println!("{}", edge.to().index()));

        let node_2 = g.get_node::<View>(2).unwrap();
        println!("IN OF 2");
        node_2
            .inedges::<Index>()
            .for_each(|edge| println!("{}", edge.from().index()));

        let t = TransposeView::of(&g);
        let node_0: NodeView<_> = node_0.same(&t);
        let node_2: NodeView<_> = node_2.same(&t);

        println!("OUT OF 2");
        node_2
            .outedges::<Index>()
            .for_each(|edge| println!("{}", edge.to().index()));

        println!("IN OF 0");
        node_0
            .inedges::<Index>()
            .for_each(|edge| println!("{}", edge.from().index()));

        // let g1 = GraphTest { successors: vec![] };
        // g1.outdegree(node_1);
    }
}
