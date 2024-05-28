use crate::graphs::Graph;

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

    fn add_triangle(&mut self, u: usize, v: usize, w: usize) {
        self.mps.add_edge(u, v);
        self.mps.add_edge(v, w);
        self.mps.add_edge(w, u);
    }

    fn remove_triangle(&mut self, u: usize, v: usize, w: usize) {
        self.full.remove_edge(u, v);
        self.full.remove_edge(v, w);
        self.full.remove_edge(w, u);
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

    fn apply_k3_rule(&mut self, x: usize, y: usize, z: usize) {
        self.add_triangle(x, y, z);
        self.remove_triangle(x, y, z);
        self.connect_components(&[x, y, z]);
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
        self.mps.add_edge(u, v);
        self.full.remove_edge(u, v);
        self.connect_components(&[u, v]);
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
