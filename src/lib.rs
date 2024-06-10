/// # Graphs
///
/// This module contains a simple graph implementation.
pub mod graphs;

/// # MPS Algorithm
///
/// This module contains traits for MPS algorithms.
pub mod mps_alg;

/// # Planarity
///
/// This module contains a planarity testing algorithm.
pub mod planarity;
// pub mod new_planarity;

/// # Random Graphs
///
/// This module contains functions for generating random graphs.
pub mod rand_graphs;

/// # Match and merge
///
/// This module contains approximation algorithms for MPS problem using match and merge framework.
pub mod match_merge;

// /// # Facial walks
// ///
// /// This module contains an exact algorithm for MPS based on facial walks.
pub mod facial_walks;

// /// # Schnyder
// ///
// /// This module contains an exact algorithm for MPS based the Schnyder poset characterization of planar graphs.
pub mod schnyder;

/// # Tests
///
/// This module contains tests for the library.
#[cfg(test)]
pub mod tests;
