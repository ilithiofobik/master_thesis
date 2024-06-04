use crate::graphs::Graph;

pub trait MpsAlgorithm {
    fn maximum_planar_subgraph(g: &Graph) -> Graph;
}
