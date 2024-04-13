use fastrand;
use std::collections::HashSet;

pub struct Graph {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbours: Vec<HashSet<usize>>,
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
            neighbours: Vec::new(),
        }
    }

    pub fn empty(num_of_vertices: usize) -> Self {
        let mut neighbours = Vec::new();
        for _ in 0..num_of_vertices {
            neighbours.push(HashSet::new());
        }

        Graph {
            num_of_vertices,
            num_of_edges: 0,
            neighbours,
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
        let mut neighbours = Vec::with_capacity(num_of_vertices);
        for i in 0..num_of_vertices {
            let set = (0..num_of_vertices)
                .filter(|&j| i != j)
                .collect::<HashSet<usize>>();

            neighbours.push(set);
        }

        Graph {
            num_of_vertices,
            num_of_edges: num_of_vertices * (num_of_vertices - 1) / 2,
            neighbours,
        }
    }

    fn is_valid_vertex(&self, vertex: usize) -> bool {
        vertex < self.num_of_vertices
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        // both vertices must be valid
        if !self.is_valid_vertex(from) || !self.is_valid_vertex(to) {
            return false;
        }

        // no self loops nor multi-edges
        if from == to || self.neighbours[from].contains(&to) {
            return false;
        }

        self.neighbours[from].insert(to);
        self.neighbours[to].insert(from);
        self.num_of_edges += 1;

        true
    }

    pub fn num_of_vertices(&self) -> usize {
        self.num_of_vertices
    }

    pub fn num_of_edges(&self) -> usize {
        self.num_of_edges
    }
}
