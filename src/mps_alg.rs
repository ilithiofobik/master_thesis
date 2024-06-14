use crate::graphs::Graph;

/// A trait representing an algorithm for finding the Maximum Planar Subgraph (MPS).
pub trait MpsAlgorithm {
    /// Computes the maximum planar subgraph of the given graph.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph;

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str;
}
