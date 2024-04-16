use crate::graphs::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::RwLock;

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

#[derive(Clone)]
struct Node {
    graph_vertex: usize,
    root_vertex: bool,
    dfs_index: usize,
    height: usize,
    lowpoint: usize,
    least_ancestor: usize,
    visited: usize,
    backedge_flag: usize,
    pertinent_roots: Vec<usize>,
    separated_dfs_children: Vec<Rc<RwLock<Node>>>,
    parent_edge: Option<Rc<RwLock<Edge>>>,
    list_node: LinkedList<Rc<RwLock<Node>>>,
    back_edges: Vec<Rc<RwLock<Edge>>>,
    tree_edges: Vec<Rc<RwLock<Edge>>>,
}

struct Edge {
    source: Rc<RwLock<Node>>,
    target: Rc<RwLock<Node>>,
    sign: isize,
    embedded: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            graph_vertex: 0,
            root_vertex: false,
            dfs_index: 0,
            height: 0,
            lowpoint: 0,
            least_ancestor: 0,
            visited: 0,
            backedge_flag: 0,
            pertinent_roots: Vec::new(),
            separated_dfs_children: Vec::new(),
            parent_edge: None,
            list_node: LinkedList::new(),
            back_edges: Vec::new(),
            tree_edges: Vec::new(),
        }
    }
}

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
    nodes: Vec<Rc<RwLock<Node>>>,
}

struct OrientDfsStackInfo {
    current: usize,
    parent: usize,
    backedge: bool,
    parent_edge: (usize, usize),
}

impl Planarity<'_> {
    pub fn new(graph: &Graph) -> Planarity {
        let n = graph.num_of_vertices();
        let nodes = vec![];
        Planarity { graph, n, nodes }
    }

    fn create_new_node(
        &mut self,
        visited: &mut Vec<Option<Node>>,
        graph_vertex: usize,
        parent_idx: usize,
        dfi: usize,
    ) -> Node {
        let mut child = Node::default();
        let parent = visited[parent_idx];

        child
    }

    fn dfs_orient(&mut self, visited: &mut Vec<Option<Node>>, v: usize, current_dfi: &mut usize) {
        let mut stack: Vec<OrientDfsStackInfo> = Vec::new();
        stack.push(OrientDfsStackInfo {
            current: v,
            parent: 0,
            backedge: false,
            parent_edge: (0, 0),
        });

        while let Some(info) = stack.pop() {
            if info.backedge {
                // process backedge
                let current = visited[info.current].as_mut().unwrap();
                current.lowpoint = current.dfs_index;
                current.least_ancestor = current.dfs_index;

                for back_edge in (*current).back_edges.iter() {
                    current.least_ancestor = std::cmp::min(
                        current.least_ancestor,
                        back_edge.read().unwrap().target.read().unwrap().dfs_index,
                    );
                }

                for tree_edge in (*current).tree_edges.iter() {
                    current.lowpoint = std::cmp::min(
                        current.least_ancestor,
                        tree_edge.read().unwrap().target.read().unwrap().lowpoint,
                    );
                }

                current.lowpoint = std::cmp::min(current.lowpoint, current.least_ancestor);
            } else {
                // process forward edge
                if visited[info.current].is_some() {
                    continue;
                }

                stack.push(OrientDfsStackInfo {
                    current: info.current,
                    parent: info.parent,
                    backedge: true,
                    parent_edge: info.parent_edge,
                });

                let current = self.create_new_node(
                    visited,
                    info.current,
                    info.parent,
                    *current_dfi,
                );
            }
        }
    }

    // sorting vertices by lowpoint
    fn sort_vertices(&mut self) {
        self.nodes
            .sort_by(|a, b| a.read().unwrap().lowpoint.cmp(&b.read().unwrap().lowpoint));

        for node in self.nodes.iter() {
            let node_write = node.write().unwrap();
            if let Some(parent_edge) = &(*node_write).parent_edge {
                let mut parent_edge_write = parent_edge.write().unwrap();
                parent_edge_write.source = Rc::clone(&node);
            }
        }
    }

    // calculating dfs and lowpoint
    fn dfs_lowpoint(&mut self) {
        let n = self.n;
        let mut visited: Vec<Option<Node>> = vec![None; n];
        let mut current_dfi = 0;

        for v in self.graph.vertices() {
            if visited[v].is_none() {
                self.dfs_orient(&mut visited, v, &mut current_dfi);
            }
        }

        self.sort_vertices();
    }

    pub fn is_planar(&mut self) -> bool {
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

        for v in (0..n).rev() {}

        true
    }
}

pub fn is_planar(graph: &Graph) -> bool {
    let mut planarity = Planarity::new(graph);
    planarity.is_planar()
}
