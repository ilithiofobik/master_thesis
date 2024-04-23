use crate::graphs::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::RwLock;

struct Node {}
struct Edge {}

struct AdjElement {
    twin: Rc<RwLock<AdjElement>>,
    node: Rc<RwLock<Node>>,
    edge: Rc<RwLock<Edge>>,
    id: usize,
}

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
}

impl Planarity<'_> {
    pub fn new(graph: &Graph) -> Planarity {
        let n = graph.num_of_vertices();
        Planarity { graph, n }
    }

    fn compute_dfs(&mut self) {
        //let mut stack = vec![];
        let mut next_dfi = 1;
        let n = self.graph.num_of_vertices();

        for v in 0..n {}
    }

    fn compute_lowpoints(&mut self) {}
    fn compute_separated_dfs_children(&mut self) {}
    fn embed(&mut self) -> bool {
        true
    }

    pub fn is_planar(&mut self) -> bool {
        self.compute_dfs();
        self.compute_lowpoints();
        self.compute_separated_dfs_children();
        self.embed()
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
