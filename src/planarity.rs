use crate::graphs::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::RwLock;
use std::vec;

const CCW: usize = 0;
const CW: usize = 1;

enum AdjacencyListLink {
    InV(usize),
    InR(usize),
    InE(usize),
}

struct HalfEdge {
    link: [AdjacencyListLink; 2],
    neighbour: usize,
    sign: isize,
}

struct RootVertex {
    link: [AdjacencyListLink; 2],
    parent: usize,
}

struct Vertex {
    link: [AdjacencyListLink; 2],
    dfs_parent: usize,
    least_ancestor: usize,
    lowpoint: usize,
    backedge_flag: usize,
    pertinent_roots: Vec<usize>,
    separated_dfs_children: Vec<Rc<RwLock<usize>>>,
    p_node_in_child_list_of_parent: Rc<RwLock<usize>>,
}

struct EmbeddingGraph {
    n: usize,
    m: usize,
    v: Vec<Vertex>,
    r: Vec<RootVertex>,
    e: Vec<HalfEdge>,
    s: Vec<usize>,
}

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
    dfi: Vec<Option<usize>>,
    lowpoint: Vec<usize>,
}

impl Planarity<'_> {
    pub fn new(graph: &Graph) -> Planarity {
        let n = graph.num_of_vertices();
        let dfi = vec![None; n];
        let lowpoint = vec![0; n];
        Planarity {
            graph,
            n,
            dfi,
            lowpoint,
        }
    }

    fn biconnect(
        &mut self,
        v: usize,
        u: usize,
        counter: &mut usize,
        edge_stack: &mut Vec<(usize, usize)>,
    ) {
        *counter += 1;
        self.dfi[v] = Some(*counter);
        self.lowpoint[v] = *counter;

        let mut sorted_neighbours_of_v = (*self.graph.neighbours(v).unwrap())
            .iter()
            .cloned()
            .collect::<Vec<usize>>();
        sorted_neighbours_of_v.sort();

        for w in sorted_neighbours_of_v {
            match self.dfi[w] {
                None => {
                    self.biconnect(w, v, counter, edge_stack);
                    self.lowpoint[v] = std::cmp::min(self.lowpoint[v], self.lowpoint[w]);
                }
                Some(dfi_w) => {
                    if dfi_w < self.dfi[v].unwrap() && w != u {
                        self.lowpoint[v] = std::cmp::min(self.lowpoint[v], dfi_w);
                    }
                }
            }
        }
    }

    fn compute_lowpoints(&mut self) {
        let mut counter = 0;
        let mut edge_stack = vec![];
        for w in self.graph.vertices() {
            if self.dfi[w].is_none() {
                self.biconnect(w, self.n + 1, &mut counter, &mut edge_stack);
            }
        }
    }

    fn initialize_embedding_graph(&mut self) {}
    fn compute_separated_dfs_children(&mut self) {}
    fn embed_edge(&mut self, u: usize, v: usize) {}
    fn embed(&mut self) -> bool {
        true
    }

    fn print_lowpoints(&self) {
        println!("Lowpoints:");
        for i in 0..self.n {
            println!("{}: {}", i, self.lowpoint[i]);
        }
    }

    fn print_dfi(&self) {
        println!("DFI:");
        for i in 0..self.n {
            println!("{}: {:?}", i, self.dfi[i]);
        }
    }

    pub fn is_planar(&mut self) -> bool {
        self.compute_lowpoints();
        self.print_lowpoints();
        self.print_dfi();

        true
    }
}

pub fn is_planar(graph: &Graph) -> bool {
    let n = graph.num_of_vertices();
    let m = graph.num_of_edges();

    // check for trivial cases
    if n <= 4 {
        return true;
    }

    if m > 3 * n - 6 {
        return false;
    }

    let mut planarity = Planarity::new(graph);
    planarity.is_planar()
}
