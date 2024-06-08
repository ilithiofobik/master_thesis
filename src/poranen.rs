use crate::graphs::Graph;
use crate::mps_alg::*;

struct MatchMerge {
    n: usize,
    full: Graph,
    mps: Graph,
    component: Vec<usize>,
    components: Vec<Vec<usize>>,
}

impl MatchMerge {
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

    fn apply_rule(&mut self, edges: &[(usize, usize)], vertices: &[usize]) {
        for &(u, v) in edges {
            self.mps.add_edge(u, v);
            self.full.remove_edge(u, v);
        }

        self.connect_components(vertices);
    }

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

    fn apply_my_rule(&mut self, x: usize, y: usize, z: usize, w: usize, t: usize) {
        self.apply_rule(
            &[(x, y), (x, z), (x, w), (x, t), (y, z), (z, w), (w, t)],
            &[x, y, z, w, t],
        );
    }

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

    fn apply_d4_rule(&mut self, x: usize, y: usize, z: usize, w: usize) {
        self.apply_rule(&[(x, y), (x, z), (x, w), (y, z), (w, z)], &[x, y, z, w]);
    }

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

    fn apply_k3_rule(&mut self, u: usize, v: usize, w: usize) {
        self.apply_rule(&[(u, v), (v, w), (w, u)], &[u, v, w]);
    }

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

    fn apply_poranen_rule(&mut self, u: usize, v: usize, w: usize) {
        self.apply_rule(&[(u, v), (u, w)], &[u, v, w]);
    }

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

    fn apply_k2_rule(&mut self, u: usize, v: usize) {
        self.apply_rule(&[(u, v)], &[u, v]);
    }
}

pub fn calinescu_basic_mps(g: &Graph) -> Graph {
    let mut mm = MatchMerge::new(g);

    while let Some((u, v, w)) = mm.find_k3_rule() {
        mm.apply_k3_rule(u, v, w);
    }

    while let Some((u, v)) = mm.find_k2_rule() {
        mm.apply_k2_rule(u, v);
    }

    mm.mps
}

pub fn schmid_d4_mps(g: &Graph) -> Graph {
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

pub fn my_mps(g: &Graph) -> Graph {
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

pub fn poranen_mps(g: &Graph) -> Graph {
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

pub struct CalinescuMps {}
pub struct SchmidMps {}
pub struct MyMps {}
pub struct PoranenMps {}

impl MpsAlgorithm for CalinescuMps {
    fn maximum_planar_subgraph(g: &Graph) -> Graph {
        calinescu_basic_mps(g)
    }
    fn name() -> &'static str {
        "Calinescu"
    }
}

impl MpsAlgorithm for SchmidMps {
    fn maximum_planar_subgraph(g: &Graph) -> Graph {
        schmid_d4_mps(g)
    }
    fn name() -> &'static str {
        "Schmid"
    }
}

impl MpsAlgorithm for MyMps {
    fn maximum_planar_subgraph(g: &Graph) -> Graph {
        my_mps(g)
    }
    fn name() -> &'static str {
        "My"
    }
}

impl MpsAlgorithm for PoranenMps {
    fn maximum_planar_subgraph(g: &Graph) -> Graph {
        poranen_mps(g)
    }
    fn name() -> &'static str {
        "Poranen"
    }
}
