use crate::graphs::Graph;
use std::collections::HashSet;

pub fn sort_3tuple<T>(a: T, b: T, c: T) -> (T, T, T)
where
    T: PartialOrd,
{
    if a <= b {
        if b <= c {
            (a, b, c)
        } else if a <= c {
            (a, c, b)
        } else {
            (c, a, b)
        }
    } else if a <= c {
        (b, a, c)
    } else if b <= c {
        (b, c, a)
    } else {
        (c, b, a)
    }
}

pub struct TriangleLister<'a> {
    graph: &'a Graph,
    marked: HashSet<usize>,
    processed: Vec<bool>,
    sorted_vertices: Vec<usize>,
    u_neighbors: Vec<usize>,
    u: usize,
    v: usize,
}

impl TriangleLister<'_> {
    pub fn new(graph: &Graph) -> TriangleLister {
        let n = graph.num_of_vertices();
        let u = 0;
        let mut v = 0;
        let mut marked = HashSet::new();
        let processed = vec![false; n];
        let u_neighbors = Vec::new();
        let mut sorted_vertices = graph.vertices().collect::<Vec<usize>>();
        sorted_vertices.sort_by_key(|b| graph.degree(*b));

        if let Some(v_sorted) = sorted_vertices.pop() {
            v = v_sorted;
            let v_neighbors = graph.neighbors(v).unwrap();
            for &u in v_neighbors {
                marked.insert(u);
            }
        }

        TriangleLister {
            graph,
            marked,
            processed,
            sorted_vertices,
            u_neighbors,
            u,
            v,
        }
    }
}

impl Iterator for TriangleLister<'_> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(w) = self.u_neighbors.pop() {
                if self.marked.contains(&w) {
                    return Some((self.v, self.u, w));
                }
            } else if !self.marked.is_empty() {
                self.u = *self.marked.iter().next().unwrap();
                self.marked.remove(&self.u);
                self.u_neighbors = self
                    .graph
                    .neighbors(self.u)
                    .unwrap()
                    .iter()
                    .copied()
                    .collect();
            } else if let Some(v) = self.sorted_vertices.pop() {
                self.processed[self.v] = true;
                self.v = v;
                let v_neighbors = self.graph.neighbors(v).unwrap();
                for &u in v_neighbors {
                    if !self.processed[u] {
                        self.marked.insert(u);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

pub fn list_triangles(g: &Graph) -> HashSet<(usize, usize, usize)> {
    let triangle_lister = TriangleLister::new(g);
    triangle_lister.collect()
}

fn new_vertex_listing(
    g: &Graph,
    v: usize,
    k: usize,
    triangles: &mut HashSet<(usize, usize, usize)>,
) {
    for &u in g.neighbors(v).unwrap() {
        let du_gt_k = g.degree(v) > k;
        for &w in g
            .neighbors(u)
            .unwrap()
            .iter()
            .filter(|&&w| g.has_edge(w, v))
        {
            let dw_gt_k = g.degree(u) > k;
            let cond1 = du_gt_k && dw_gt_k && v > u && u > w;
            let cond2 = du_gt_k && !dw_gt_k && v > u;
            let cond3 = !du_gt_k && dw_gt_k && v > w;
            if cond1 || cond2 || cond3 {
                triangles.insert((w, u, v));
            }
        }
    }
}

fn edge_listing(g: &Graph, u: usize, v: usize, triangles: &mut HashSet<(usize, usize, usize)>) {
    for &w in g
        .neighbors(u)
        .unwrap()
        .iter()
        .filter(|&&w| g.has_edge(v, w) && v < w)
    {
        triangles.insert((u, v, w));
    }
}

fn new_listing_with_k(g: &Graph, k: usize) -> HashSet<(usize, usize, usize)> {
    let mut triangles = HashSet::new();

    for v in g.vertices().filter(|&v| g.degree(v) > k) {
        new_vertex_listing(g, v, k, &mut triangles);
    }

    for u in g.vertices().filter(|&u| g.degree(u) <= k) {
        for &v in g
            .neighbors(u)
            .unwrap()
            .iter()
            .filter(|&&v| g.degree(v) <= k && u < v)
        {
            edge_listing(g, u, v, &mut triangles);
        }
    }

    triangles
}

pub fn new_listing(g: &Graph) -> HashSet<(usize, usize, usize)> {
    let n: f64 = g.num_of_vertices() as f64;
    let k = n.sqrt().round() as usize;
    new_listing_with_k(g, k)
}
