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

/// # Random Graphs
///
/// This module contains functions for generating random graphs.
pub mod rand_graphs;

/// # Triangle Listing
///
/// This module contains an algorithm for listing all triangles in a graph.
pub mod triangle_listing;

/// # Cacti
///
/// This module contains approximation algorithm for MPS problem using cacti algorithms.
pub mod cacti;

/// # Poranen
///
/// This module contains approximation algorithm for MPS problem using Poranen's algorithm.
pub mod poranen;

/// # Facial walks
///
/// This module contains an exact algorithm for MPS based on facial walks.
pub mod facial_walks;

/// # Schnyder
///
/// This module contains an exact algorithm for MPS based the Schnyder poset characterization of planar graphs.
pub mod schnyder;

/// # Union find
///
/// This module contains a union-find data structure.
pub mod union_find;

/// # Tests
///
/// This module contains tests for the library.
#[cfg(test)]
pub mod tests;
