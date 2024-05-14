use crate::graphs::Graph;
use std::{collections::HashSet, hash::Hash};

fn sort_3tuple<T>(a: T, b: T, c: T) -> (T, T, T)
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

pub fn list_triangles(g: &Graph) -> HashSet<(usize, usize, usize)> {
    let n = g.num_of_vertices();
    let mut triangles = HashSet::new();
    let mut marked = HashSet::new();
    let mut processed = vec![false; n];

    let mut sorted_vertices = g.vertices().collect::<Vec<usize>>();
    sorted_vertices.sort_by_key(|b| std::cmp::Reverse(g.degree(*b)));

    while let Some(v) = sorted_vertices.pop() {
        let v_neighbors = g.neighbors(v).unwrap();
        for u in v_neighbors {
            if !processed[*u] {
                marked.insert(u);
            }
        }

        while !marked.is_empty() {
            let u = *marked.iter().next().unwrap();
            marked.remove(u);
            let u_neighbors = g.neighbors(*u).unwrap();

            for w in u_neighbors {
                if marked.contains(w) {
                    let triangle = sort_3tuple(v, *u, *w);
                    triangles.insert(triangle);
                }
            }
        }

        processed[v] = true;
    }

    triangles
}

fn new_vertex_listing(
    g: &Graph,
    v: usize,
    k: usize,
    triangles: &mut HashSet<(usize, usize, usize)>,
) {
    for &u in g.neighbors(v).unwrap() {
        let u_gt_k = g.degree(v) > k;
        for &w in g.neighbors(v).unwrap() {
            let w_gt_k = g.degree(u) > k;
            if g.has_edge(v, w) && (w_gt_k || u_gt_k) {
                triangles.insert(sort_3tuple(u, v, w));
            }
        }
    }
}

fn edge_listing(g: &Graph, u: usize, v: usize, triangles: &mut HashSet<(usize, usize, usize)>) {
    for &w in g.neighbors(u).unwrap() {
        if g.has_edge(v, w) {
            if w <= v {
                continue;
            }
            triangles.insert(sort_3tuple(u, v, w));
        }
    }
}

fn new_listing_with_k(g: &Graph, k: usize) -> HashSet<(usize, usize, usize)> {
    let mut triangles = HashSet::new();

    for v in g.vertices() {
        if g.degree(v) > k {
            new_vertex_listing(g, v, k, &mut triangles);
        }
    }

    for u in g.vertices() {
        for &v in g.neighbors(u).unwrap() {
            if u < v {
                edge_listing(g, u, v, &mut triangles);
            }
        }
    }

    triangles
}

pub fn new_listing(g: &Graph) -> HashSet<(usize, usize, usize)> {
    let n = g.num_of_vertices() as f64;
    let k = n.sqrt().round() as usize;
    new_listing_with_k(g, k)
}
