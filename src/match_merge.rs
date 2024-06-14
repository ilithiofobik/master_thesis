use crate::graphs::Graph;
use crate::mps_alg::*;

/// A struct representing the Match and Merge algorithm for finding the maximum planar subgraph.
struct MatchMerge {
    n: usize,
    full: Graph,
    mps: Graph,
    component: Vec<usize>,
    components: Vec<Vec<usize>>,
}

impl MatchMerge {
    /// Creates a new MatchMerge instance.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * A new instance of `MatchMerge`.
    pub fn new(g: &Graph) -> MatchMerge {
        let n = g.num_of_vertices();
        let full = g.clone();
        let mps = Graph::empty(n);
        let component = (0..n).collect::<Vec<usize>>();
        let components = (0..n).map(|x| vec![x]).collect::<Vec<Vec<usize>>>();

        MatchMerge {
            n,
            full,
            mps,
            component,
            components,
        }
    }

    /// Checks if all vertices belong to different components.
    ///
    /// # Arguments
    /// * `vertices` - A slice of vertex indices.
    ///
    /// # Returns
    /// * `true` if all vertices are in different components, `false` otherwise.
    fn all_different_components(&self, vertices: &[usize]) -> bool {
        let n = vertices.len();

        for i in 0..n {
            for j in i + 1..n {
                if self.component[vertices[i]] == self.component[vertices[j]] {
                    return false;
                }
            }
        }

        true
    }

    /// Connects the components of the given vertices.
    ///
    /// # Arguments
    /// * `vs` - A slice of vertex indices.
    fn connect_components(&mut self, vs: &[usize]) {
        if vs.len() <= 1 {
            return;
        }

        let n = vs.len();
        let u = vs[0];
        let u_comp = self.component[u];

        for v in 1..n {
            let v_comp = self.component[vs[v]];

            if v_comp == u_comp {
                continue;
            }

            while let Some(x) = self.components[v_comp].pop() {
                self.component[x] = u_comp;
                self.components[u_comp].push(x);
            }
        }
    }

    /// Applies a given rule by adding edges and connecting components.
    ///
    /// # Arguments
    /// * `edges` - A slice of edge tuples to be added to the MPS.
    /// * `vertices` - A slice of vertex indices involved in the rule.
    fn apply_rule(&mut self, edges: &[(usize, usize)], vertices: &[usize]) {
        for &(u, v) in edges {
            self.mps.add_edge(u, v);
            self.full.remove_edge(u, v);
        }

        self.connect_components(vertices);
    }

    /// Finds a set of vertices that match the custom rule.
    ///
    /// # Returns
    /// * An optional tuple of five vertex indices if the rule is matched, or `None` otherwise.
    fn find_my_rule(&self) -> Option<(usize, usize, usize, usize, usize)> {
        for x in 0..self.n {
            let x_neighbors = self.full.neighbors(x).unwrap();
            for &y in x_neighbors {
                for &z in x_neighbors {
                    if y == z || !self.full.has_edge(y, z) {
                        continue;
                    }

                    for &w in x_neighbors {
                        if w == y || w == z || !self.full.has_edge(z, w) {
                            continue;
                        }

                        for &t in x_neighbors {
                            if t == y || t == z || t == w || !self.full.has_edge(w, t) {
                                continue;
                            }

                            if !self.all_different_components(&[x, y, z, w, t]) {
                                continue;
                            }

                            return Some((x, y, z, w, t));
                        }
                    }
                }
            }
        }

        None
    }

    /// Applies the custom rule by adding edges and connecting components.
    ///
    /// # Arguments
    /// * `x` - Vertex index.
    /// * `y` - Vertex index.
    /// * `z` - Vertex index.
    /// * `w` - Vertex index.
    /// * `t` - Vertex index.
    fn apply_my_rule(&mut self, x: usize, y: usize, z: usize, w: usize, t: usize) {
        self.apply_rule(
            &[(x, y), (x, z), (x, w), (x, t), (y, z), (z, w), (w, t)],
            &[x, y, z, w, t],
        );
    }

    /// Finds a set of vertices that match the D4 rule.
    ///
    /// # Returns
    /// * An optional tuple of four vertex indices if the rule is matched, or `None` otherwise.
    fn find_d4_rule(&self) -> Option<(usize, usize, usize, usize)> {
        for x in 0..self.n {
            let x_neighbors = self.full.neighbors(x).unwrap();
            for &y in x_neighbors {
                for &z in x_neighbors {
                    for &w in x_neighbors {
                        if y == z || y == w || z == w {
                            continue;
                        }

                        if !self.full.has_edge(y, z) || !self.full.has_edge(z, w) {
                            continue;
                        }

                        if !self.all_different_components(&[x, y, z, w]) {
                            continue;
                        }

                        return Some((x, y, z, w));
                    }
                }
            }
        }

        None
    }

    /// Applies the D4 rule by adding edges and connecting components.
    ///
    /// # Arguments
    /// * `x` - Vertex index.
    /// * `y` - Vertex index.
    /// * `z` - Vertex index.
    /// * `w` - Vertex index.
    fn apply_d4_rule(&mut self, x: usize, y: usize, z: usize, w: usize) {
        self.apply_rule(&[(x, y), (x, z), (x, w), (y, z), (w, z)], &[x, y, z, w]);
    }

    /// Finds a set of vertices that match the K3 rule.
    ///
    /// # Returns
    /// * An optional tuple of three vertex indices if the rule is matched, or `None` otherwise.
    fn find_k3_rule(&self) -> Option<(usize, usize, usize)> {
        for x in 0..self.n {
            let x_neighbors = self.full.neighbors(x).unwrap();
            for &y in x_neighbors {
                for &z in x_neighbors {
                    if y == z {
                        continue;
                    }

                    if !self.full.has_edge(y, z) {
                        continue;
                    }

                    if !self.all_different_components(&[x, y, z]) {
                        continue;
                    }

                    return Some((x, y, z));
                }
            }
        }

        None
    }

    /// Applies the K3 rule by adding edges and connecting components.
    ///
    /// # Arguments
    /// * `u` - Vertex index.
    /// * `v` - Vertex index.
    /// * `w` - Vertex index.
    fn apply_k3_rule(&mut self, u: usize, v: usize, w: usize) {
        self.apply_rule(&[(u, v), (v, w), (w, u)], &[u, v, w]);
    }

    /// Finds a set of vertices that match the Poranen rule.
    ///
    /// # Returns
    /// * An optional tuple of three vertex indices if the rule is matched, or `None` otherwise.
    fn find_poranen_rule(&self) -> Option<(usize, usize, usize)> {
        for x in 0..self.n {
            let x_neighbors = self.full.neighbors(x).unwrap();
            for &y in x_neighbors {
                for &z in x_neighbors {
                    if y == z {
                        continue;
                    }

                    if !self.mps.has_edge(y, z) {
                        continue;
                    }

                    if !self.all_different_components(&[x, y]) {
                        continue;
                    }

                    return Some((x, y, z));
                }
            }
        }

        None
    }

    /// Applies the Poranen rule by adding edges and connecting components.
    ///
    /// # Arguments
    /// * `u` - Vertex index.
    /// * `v` - Vertex index.
    /// * `w` - Vertex index.
    fn apply_poranen_rule(&mut self, u: usize, v: usize, w: usize) {
        self.apply_rule(&[(u, v), (u, w)], &[u, v, w]);
    }

    /// Finds a set of vertices that match the K2 rule.
    ///
    /// # Returns
    /// * An optional tuple of two vertex indices if the rule is matched, or `None` otherwise.
    fn find_k2_rule(&self) -> Option<(usize, usize)> {
        for x in 0..self.n {
            let x_neighbors = self.full.neighbors(x).unwrap();
            for &y in x_neighbors {
                if !self.all_different_components(&[x, y]) {
                    continue;
                }

                return Some((x, y));
            }
        }

        None
    }

    /// Applies the K2 rule by adding an edge and connecting components.
    ///
    /// # Arguments
    /// * `u` - Vertex index.
    /// * `v` - Vertex index.
    fn apply_k2_rule(&mut self, u: usize, v: usize) {
        self.apply_rule(&[(u, v)], &[u, v]);
    }
}

/// A struct representing the Calinescu MPS algorithm.
pub struct CalinescuMps {}

/// A struct representing the Schmid MPS algorithm.
pub struct SchmidMps {}

/// A struct representing a custom MPS algorithm.
pub struct MyMps {}

/// A struct representing the Poranen MPS algorithm.
pub struct PoranenMps {}

impl MpsAlgorithm for CalinescuMps {
    /// Computes the maximum planar subgraph using the Calinescu algorithm.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        let mut mm = MatchMerge::new(g);

        while let Some((u, v, w)) = mm.find_k3_rule() {
            mm.apply_k3_rule(u, v, w);
        }

        while let Some((u, v)) = mm.find_k2_rule() {
            mm.apply_k2_rule(u, v);
        }

        mm.mps
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "Calinescu"
    }
}

impl MpsAlgorithm for SchmidMps {
    /// Computes the maximum planar subgraph using the Schmid algorithm.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        let mut mm = MatchMerge::new(g);

        while let Some((x, y, z, w)) = mm.find_d4_rule() {
            mm.apply_d4_rule(x, y, z, w);
        }

        while let Some((u, v, w)) = mm.find_k3_rule() {
            mm.apply_k3_rule(u, v, w);
        }

        while let Some((u, v)) = mm.find_k2_rule() {
            mm.apply_k2_rule(u, v);
        }

        mm.mps
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "Schmid"
    }
}

impl MpsAlgorithm for MyMps {
    /// Computes the maximum planar subgraph using a custom algorithm.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        let mut mm = MatchMerge::new(g);

        while let Some((x, y, z, w, t)) = mm.find_my_rule() {
            mm.apply_my_rule(x, y, z, w, t);
        }

        while let Some((x, y, z, w)) = mm.find_d4_rule() {
            mm.apply_d4_rule(x, y, z, w);
        }

        while let Some((u, v, w)) = mm.find_k3_rule() {
            mm.apply_k3_rule(u, v, w);
        }

        while let Some((u, v)) = mm.find_k2_rule() {
            mm.apply_k2_rule(u, v);
        }

        mm.mps
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "My"
    }
}

impl MpsAlgorithm for PoranenMps {
    /// Computes the maximum planar subgraph using the Poranen algorithm.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        let mut mm = MatchMerge::new(g);
        let mut found = true;

        while found {
            found = false;

            while let Some((x, y, z)) = mm.find_poranen_rule() {
                mm.apply_poranen_rule(x, y, z);
                found = true;
            }

            if let Some((u, v, w)) = mm.find_k3_rule() {
                mm.apply_k3_rule(u, v, w);
                found = true;
            }
        }

        while let Some((u, v)) = mm.find_k2_rule() {
            mm.apply_k2_rule(u, v);
        }

        mm.mps
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "Poranen"
    }
}
