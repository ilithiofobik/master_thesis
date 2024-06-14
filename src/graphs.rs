use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::fs::File;
use std::ops::Add;
use std::ops::Range;

/// A struct representing a graph with a given number of vertices and edges.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbors: Vec<HashSet<usize>>,
}

/// A struct for serializing and deserializing a graph to/from JSON.
#[derive(Serialize, Deserialize)]
struct GraphJson {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbors: Vec<Vec<usize>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Graph {
    type Output = Self;

    /// Adds two graphs together by combining their vertices and edges.
    ///
    /// # Arguments
    /// * `self` - The first graph.
    /// * `other` - The second graph.
    ///
    /// # Returns
    /// * A new graph that is the result of combining the vertices and edges of both graphs.
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
    /// Creates a new, empty graph.
    ///
    /// # Returns
    /// * A new instance of an empty `Graph`.
    pub fn new() -> Self {
        Graph {
            num_of_vertices: 0,
            num_of_edges: 0,
            neighbors: Vec::new(),
        }
    }

    /// Creates an empty graph with a specified number of vertices.
    ///
    /// # Arguments
    /// * `num_of_vertices` - The number of vertices in the graph.
    ///
    /// # Returns
    /// * A new instance of an empty `Graph` with the specified number of vertices.
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

    /// Creates a complete graph with a specified number of vertices.
    ///
    /// # Arguments
    /// * `num_of_vertices` - The number of vertices in the complete graph.
    ///
    /// # Returns
    /// * A new instance of a complete `Graph` with the specified number of vertices.
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

    /// Creates a bipartite complete graph with specified partitions.
    ///
    /// # Arguments
    /// * `n` - The number of vertices in the first partition.
    /// * `m` - The number of vertices in the second partition.
    ///
    /// # Returns
    /// * A new instance of a bipartite complete `Graph` with the specified partitions.
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

    /// Checks if a vertex is valid (i.e., within the range of the graph's vertices).
    ///
    /// # Arguments
    /// * `vertex` - The index of the vertex.
    ///
    /// # Returns
    /// * `true` if the vertex is valid, `false` otherwise.
    fn is_valid_vertex(&self, vertex: usize) -> bool {
        vertex < self.num_of_vertices
    }

    /// Determines if the graph is planar.
    ///
    /// # Returns
    /// * `true` if the graph is planar, `false` otherwise.
    pub fn is_planar(&self) -> bool {
        let n = self.num_of_vertices();
        let m = self.num_of_edges();

        if n <= 4 {
            return true;
        }

        if m > 3 * n - 6 {
            return false;
        }

        let edges = self
            .all_edges()
            .iter()
            .map(|&(u, v)| (u as u32, v as u32))
            .collect::<Vec<_>>();
        let petgraph = rustworkx_core::petgraph::graph::UnGraph::<usize, ()>::from_edges(&edges);
        rustworkx_core::planar::is_planar(&petgraph)
    }

    /// Checks if an edge exists between two vertices.
    ///
    /// # Arguments
    /// * `from` - The index of the starting vertex.
    /// * `to` - The index of the ending vertex.
    ///
    /// # Returns
    /// * `true` if the edge exists, `false` otherwise.
    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.neighbors[from].contains(&to)
    }

    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// * The index of the newly added vertex.
    pub fn add_vertex(&mut self) -> usize {
        self.neighbors.push(HashSet::new());
        self.num_of_vertices += 1;
        self.num_of_vertices - 1
    }

    /// Adds an edge between two vertices.
    ///
    /// # Arguments
    /// * `from` - The index of the starting vertex.
    /// * `to` - The index of the ending vertex.
    ///
    /// # Returns
    /// * `true` if the edge is successfully added, `false` otherwise.
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

    /// Removes an edge between two vertices.
    ///
    /// # Arguments
    /// * `from` - The index of the starting vertex.
    /// * `to` - The index of the ending vertex.
    ///
    /// # Returns
    /// * `true` if the edge is successfully removed, `false` otherwise.
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

    /// Returns the number of vertices in the graph.
    ///
    /// # Returns
    /// * The number of vertices in the graph.
    pub fn num_of_vertices(&self) -> usize {
        self.num_of_vertices
    }

    /// Returns an iterator over the range of vertices.
    ///
    /// # Returns
    /// * An iterator over the range of vertices in the graph.
    pub fn vertices(&self) -> Range<usize> {
        0..self.num_of_vertices
    }

    /// Returns the number of edges in the graph.
    ///
    /// # Returns
    /// * The number of edges in the graph.
    pub fn num_of_edges(&self) -> usize {
        self.num_of_edges
    }

    /// Returns the neighbors of a given vertex.
    ///
    /// # Arguments
    /// * `vertex` - The index of the vertex.
    ///
    /// # Returns
    /// * An `Option` containing a reference to a `HashSet` of neighbor vertices if the vertex is valid, `None` otherwise.
    pub fn neighbors(&self, vertex: usize) -> Option<&HashSet<usize>> {
        if self.is_valid_vertex(vertex) {
            Some(&self.neighbors[vertex])
        } else {
            None
        }
    }

    /// Returns the degree of a given vertex. Returns 0 for non-existing vertices.
    ///
    /// # Arguments
    /// * `vertex` - The index of the vertex.
    ///
    /// # Returns
    /// * The degree of the vertex.
    pub fn degree(&self, vertex: usize) -> usize {
        if self.is_valid_vertex(vertex) {
            self.neighbors[vertex].len()
        } else {
            0
        }
    }

    /// Returns a vector of all edges in the graph.
    ///
    /// # Returns
    /// * A vector of tuples representing all edges in the graph.
    pub fn all_edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter().filter(|&to| from < *to) {
                edges.push((from, *to));
            }
        }
        edges
    }

    /// Returns a vector of all arcs (directed edges) in the graph.
    ///
    /// # Returns
    /// * A vector of tuples representing all arcs in the graph.
    pub fn all_arcs(&self) -> Vec<(usize, usize)> {
        let mut arcs = Vec::new();
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter() {
                arcs.push((from, *to));
            }
        }
        arcs
    }

    /// Prints all edges in the graph.
    pub fn print_edges(&self) {
        println!("Edges:");
        for (from, tos) in self.neighbors.iter().enumerate() {
            for to in tos.iter().filter(|&to| from < *to) {
                println!("{} <-> {}", from, to);
            }
        }
    }

    /// Writes the graph to a JSON file.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to write the graph to.
    ///
    /// # Returns
    /// * A `Result` indicating the success or failure of the operation.
    pub fn write_to_json(&self, filename: &str) -> serde_json::Result<()> {
        let graph = json!({
            "num_of_vertices": self.num_of_vertices,
            "num_of_edges": self.num_of_edges,
            "neighbors": self.neighbors.iter().map(|set| set.iter().cloned().collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>(),
        });
        serde_json::to_writer(&File::create(filename).unwrap(), &graph)
    }

    /// Reads a graph from a JSON file.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to read the graph from.
    ///
    /// # Returns
    /// * A new instance of `Graph` read from the file.
    pub fn read_from_json(filename: &str) -> Graph {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        let json: serde_json::Value =
            serde_json::from_str(&data).expect("JSON does not have correct format.");

        let num_of_vertices = json["num_of_vertices"].as_u64().unwrap() as usize;
        let num_of_edges = json["num_of_edges"].as_u64().unwrap() as usize;

        let neighbors = json["neighbors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_u64().unwrap() as usize)
                    .collect::<HashSet<usize>>()
            })
            .collect::<Vec<HashSet<usize>>>();

        Graph {
            num_of_vertices,
            num_of_edges,
            neighbors,
        }
    }
}
