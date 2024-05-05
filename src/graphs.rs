use core::num;
use fastrand;
use std::collections::HashSet;
use std::ops::Range;

pub struct Graph {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbors: Vec<HashSet<usize>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
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

    pub fn random(num_of_vertices: usize, num_of_edges: usize) -> Self {
        let mut graph = Graph::empty(num_of_vertices);

        let mut edges = (0..num_of_vertices)
            .flat_map(|from| (from + 1..num_of_vertices).map(move |to| (from, to)))
            .collect::<Vec<(usize, usize)>>();

        fastrand::shuffle(&mut edges);

        (0..num_of_edges).map(|i| edges[i]).for_each(|(from, to)| {
            graph.add_edge(from, to);
        });

        graph
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
            neighbors: neighbors,
        }
    }

    fn is_valid_vertex(&self, vertex: usize) -> bool {
        vertex < self.num_of_vertices
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.neighbors[from].contains(&to)
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

    pub fn print_edges(&self) {
        println!("Edges:");
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter().filter(|&to| from < *to) {
                println!("{} <-> {}", from, to);
            }
        }
    }
}
