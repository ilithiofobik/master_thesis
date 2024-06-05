use std::collections::HashSet;
use std::ops::Add;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Graph {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbors: Vec<HashSet<usize>>,
}

pub struct DirectedGraph {
    num_of_vertices: usize,
    num_of_edges: usize,
    in_neighbors: Vec<HashSet<usize>>,
    out_neighbors: Vec<HashSet<usize>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Graph {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut graph = self.clone();
        let n1 = self.num_of_vertices;
        let n2 = other.num_of_vertices;

        for _ in 0..n2 {
            graph.add_vertex();
        }

        for (from, tos) in other.neighbors.iter().enumerate() {
            for to in tos.iter() {
                graph.add_edge(from + n1, *to + n1);
            }
        }

        graph
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            num_of_vertices: 0,
            num_of_edges: 0,
            neighbors: Vec::new(),
        }
    }

    pub fn empty(num_of_vertices: usize) -> Self {
        let mut neighbors = Vec::new();
        for _ in 0..num_of_vertices {
            neighbors.push(HashSet::new());
        }

        Graph {
            num_of_vertices,
            num_of_edges: 0,
            neighbors,
        }
    }

    pub fn complete(num_of_vertices: usize) -> Self {
        if num_of_vertices == 0 {
            return Graph::empty(0);
        }

        let mut neighbors = Vec::with_capacity(num_of_vertices);
        for i in 0..num_of_vertices {
            let set = (0..num_of_vertices)
                .filter(|&j| i != j)
                .collect::<HashSet<usize>>();

            neighbors.push(set);
        }

        Graph {
            num_of_vertices,
            num_of_edges: num_of_vertices * (num_of_vertices - 1) / 2,
            neighbors,
        }
    }

    pub fn bipartite_complete(n: usize, m: usize) -> Self {
        let num_of_vertices = n + m;
        let num_of_edges = n * m;

        if num_of_edges == 0 {
            return Graph::empty(num_of_vertices);
        }

        let mut neighbors = Vec::with_capacity(num_of_vertices);
        let m_set = (n..num_of_vertices).collect::<HashSet<usize>>();
        for _ in 0..n {
            neighbors.push(m_set.clone());
        }
        let n_set = (0..n).collect::<HashSet<usize>>();
        for _ in n..num_of_vertices {
            neighbors.push(n_set.clone());
        }

        Graph {
            num_of_vertices,
            num_of_edges,
            neighbors,
        }
    }

    fn is_valid_vertex(&self, vertex: usize) -> bool {
        vertex < self.num_of_vertices
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.neighbors[from].contains(&to)
    }

    pub fn add_vertex(&mut self) -> usize {
        self.neighbors.push(HashSet::new());
        self.num_of_vertices += 1;
        self.num_of_vertices - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        // both vertices must be valid
        if !self.is_valid_vertex(from) || !self.is_valid_vertex(to) {
            println!("Invalid vertices: {} -> {}", from, to);
            return false;
        }

        // no self loops nor multi-edges
        if from == to || self.neighbors[from].contains(&to) {
            println!("Self loop or multi-edge: {} -> {}", from, to);
            return false;
        }

        self.neighbors[from].insert(to);
        self.neighbors[to].insert(from);
        self.num_of_edges += 1;

        true
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        // both vertices must be valid
        if !self.is_valid_vertex(from) || !self.is_valid_vertex(to) {
            println!("Invalid vertices: {} -> {}", from, to);
            return false;
        }

        // must exist
        if !self.neighbors[from].contains(&to) {
            println!("Edge does not exist: {} -> {}", from, to);
            return false;
        }

        self.neighbors[from].remove(&to);
        self.neighbors[to].remove(&from);
        self.num_of_edges -= 1;

        true
    }

    pub fn num_of_vertices(&self) -> usize {
        self.num_of_vertices
    }

    pub fn vertices(&self) -> Range<usize> {
        0..self.num_of_vertices
    }

    pub fn num_of_edges(&self) -> usize {
        self.num_of_edges
    }

    pub fn neighbors(&self, vertex: usize) -> Option<&HashSet<usize>> {
        if self.is_valid_vertex(vertex) {
            Some(&self.neighbors[vertex])
        } else {
            None
        }
    }

    /// Unsafe degree function - returns 0 for non-existing vertices
    pub fn degree(&self, vertex: usize) -> usize {
        if self.is_valid_vertex(vertex) {
            self.neighbors[vertex].len()
        } else {
            0
        }
    }

    pub fn all_edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter().filter(|&to| from < *to) {
                edges.push((from, *to));
            }
        }
        edges
    }

    pub fn print_edges(&self) {
        println!("Edges:");
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter().filter(|&to| from < *to) {
                println!("{} <-> {}", from, to);
            }
        }
    }
}

impl Default for DirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectedGraph {
    pub fn new() -> Self {
        DirectedGraph {
            num_of_vertices: 0,
            num_of_edges: 0,
            in_neighbors: Vec::new(),
            out_neighbors: Vec::new(),
        }
    }

    pub fn empty(num_of_vertices: usize) -> Self {
        let mut in_neighbors = Vec::new();
        let mut out_neighbors = Vec::new();
        for _ in 0..num_of_vertices {
            in_neighbors.push(HashSet::new());
            out_neighbors.push(HashSet::new());
        }

        DirectedGraph {
            num_of_vertices,
            num_of_edges: 0,
            in_neighbors,
            out_neighbors,
        }
    }

    fn is_valid_vertex(&self, vertex: usize) -> bool {
        vertex < self.num_of_vertices
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.out_neighbors[from].contains(&to)
    }

    pub fn add_vertex(&mut self) -> usize {
        self.in_neighbors.push(HashSet::new());
        self.out_neighbors.push(HashSet::new());
        self.num_of_vertices += 1;
        self.num_of_vertices - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        // both vertices must be valid
        if !self.is_valid_vertex(from) || !self.is_valid_vertex(to) {
            println!("Invalid vertices: {} -> {}", from, to);
            return false;
        }

        // no self loops nor multi-edges
        if from == to || self.out_neighbors[from].contains(&to) {
            println!("Self loop or multi-edge: {} -> {}", from, to);
            return false;
        }

        self.out_neighbors[from].insert(to);
        self.in_neighbors[to].insert(from);
        self.num_of_edges += 1;

        true
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        // both vertices must be valid
        if !self.is_valid_vertex(from) || !self.is_valid_vertex(to) {
            println!("Invalid vertices: {} -> {}", from, to);
            return false;
        }

        // must exist
        if !self.out_neighbors[from].contains(&to) {
            println!("Edge does not exist: {} -> {}", from, to);
            return false;
        }

        self.out_neighbors[from].remove(&to);
        self.in_neighbors[to].remove(&from);
        self.num_of_edges -= 1;

        true
    }

    pub fn num_of_vertices(&self) -> usize {
        self.num_of_vertices
    }

    pub fn vertices(&self) -> Range<usize> {
        0..self.num_of_vertices
    }

    pub fn num_of_edges(&self) -> usize {
        self.num_of_edges
    }

    pub fn in_neighbors(&self, vertex: usize) -> Option<&HashSet<usize>> {
        if self.is_valid_vertex(vertex) {
            Some(&self.in_neighbors[vertex])
        } else {
            None
        }
    }

    pub fn out_neighbors(&self, vertex: usize) -> Option<&HashSet<usize>> {
        if self.is_valid_vertex(vertex) {
            Some(&self.out_neighbors[vertex])
        } else {
            None
        }
    }
}
