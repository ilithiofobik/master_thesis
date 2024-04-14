use crate::graphs::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

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

struct EmbeddingGraph {
    n: usize,
    m: usize,
    v_arr: Vec<usize>,
}

struct Node {}

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
}

struct OrientDfsStackInfo {
    current: usize, 
    parent: usize,
    backedge: bool,
}

impl Planarity<'_> {
    pub fn new(graph: &Graph) -> Planarity {
        let n = graph.num_of_vertices();
        Planarity { graph, n }
    }

    fn dfs_orient(
        &self,
        visited: &mut HashMap<usize, Node>,
        v: usize,
        current_dfi: usize,
    ) -> usize {
        let mut stack: Vec<OrientDfsStackInfo> = Vec::new();
        stack.push (OrientDfsStackInfo {
            current: v,
            parent: 0,
            backedge: false,
        });

        while let Some(info) = stack.pop() {
            
        }


        0
    }

    fn dfs_lowpoint(&self) {
        let n = self.n;
        let mut visited: HashMap<usize, Node> = HashMap::with_capacity(n);
        let mut current_dfi = 0;

        for v in self.graph.vertices() {
            if !visited.contains_key(&v) {
                current_dfi = self.dfs_orient(&mut visited, v, current_dfi);
            }
        }


    }

    pub fn is_planar(&self) -> bool {
        let n = self.graph.num_of_vertices();
        let m = self.graph.num_of_edges();

        // step 0: check for trivial cases
        if n <= 4 {
            return true;
        }

        if m > 3 * n - 6 {
            return false;
        }

        // step 1: perform dfs and lowpoint calculations
        self.dfs_lowpoint();

        true
    }
}

pub fn is_planar(graph: &Graph) -> bool {
    let planarity = Planarity::new(graph);
    planarity.is_planar()
}
