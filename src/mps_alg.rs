use crate::graphs::Graph;

pub trait MpsAlgorithm {
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph;
    fn name(&self) -> &'static str;
}
