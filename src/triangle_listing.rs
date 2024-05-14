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

// 1aa. output all triangles {v, u, w} such that d(u) > K, d(w) > K and v > u > w
// 1ab. output all triangles {v, u, w} such that d(u) > K, d(w) ≤ K and v > u
// 1ac. output all triangles {v, u, w} such that d(u) ≤ K, d(w) > K and v > w

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
