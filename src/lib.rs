/// # Graphs
///
/// This module contains a simple graph implementation.
pub mod graphs;

/// # MPS Algorithm
///
/// This module contains traits and interfaces for implementing Maximum Planar Subgraph (MPS) algorithms.
pub mod mps_alg;

/// # Random Graphs
///
/// This module contains functions for generating random graphs.
pub mod rand_graphs;

/// # Match and Merge
///
/// This module contains approximation algorithms for the MPS problem using the match and merge framework.
pub mod match_merge;

/// # Facial Walks
///
/// This module contains an exact algorithm for the MPS problem based on facial walks.
pub mod facial_walks;

/// # Schnyder
///
/// This module contains an exact algorithm for the MPS problem based on the Schnyder poset characterization of planar graphs.
pub mod schnyder;

/// # Tests
///
/// This module contains tests for the library.
#[cfg(test)]
pub mod tests;
