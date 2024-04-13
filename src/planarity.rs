use crate::graphs::Graph;

enum AdjacencyListLink {
    V(usize),
    E(usize),
    R(usize),
}

struct RootVertex {
    link: (AdjacencyListLink, AdjacencyListLink),
    parent: usize,
}

struct HalfEdge {
    link: (AdjacencyListLink, AdjacencyListLink),
    neighbor: usize,
    sign: bool, // true == +, false == -
}

struct VertexInfo {
    dfs_parent: usize,
    least_ancestor: usize,
    lowpoint: usize,
    visited: usize,
    backedge_flag: usize,
    pertinent_roots: Vec<usize>,
    separated_dfs_children: Vec<usize>,
    p_node_in_children_of_partent: usize, // TODO: should be a pointer?
}

pub fn is_planar(graph: &Graph) -> bool {
    if graph.num_of_vertices() <= 4 {
        return true;
    }

    if graph.num_of_edges() > 3 * graph.num_of_vertices() - 6 {
        return false;
    }

    // step 1: perform dfs and lowpoint calculations

    true
}
