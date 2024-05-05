extern crate ndarray;
extern crate ndarray_linalg;

use crate::graphs::Graph;
use fastrand;
use itertools::*;
use ndarray::Array2;
use ndarray_linalg::solve::Inverse;
use std::collections::HashMap;
use std::collections::HashSet;

type M = Array2<f32>;

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

fn triangles_to_cactus(n: usize, triangles: &HashSet<(usize, usize, usize)>) -> Graph {
    let mut cactus = Graph::empty(n);
    for &(a, b, c) in triangles {
        cactus.add_edge(a, b);
        cactus.add_edge(b, c);
        cactus.add_edge(c, a);
    }
    cactus
}

fn _slices_concat3(p: &[usize], r: &[usize], c: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(p.len() + r.len() + c.len());
    result.extend_from_slice(p);
    result.extend_from_slice(r);
    result.extend_from_slice(c);
    result.sort();
    result.dedup();
    result
}

// fn _recalc_on_s(y_mat: &mut M, n_mat: &mut M, s: &[usize]) {}

fn triangles_to_indeterminates(
    triangles: &HashSet<(usize, usize, usize)>,
) -> HashMap<(usize, usize, usize), f32> {
    let mut indeterminates = HashMap::new();
    for triangle in triangles {
        let r = fastrand::u16(1..) as f32; // values between 1 and 2^16-1
        indeterminates.insert(*triangle, r);
    }
    indeterminates
}

fn add_triangle_to_y(y: &mut M, triangle: (usize, usize, usize), x: f32) {
    let (i, j, k) = triangle;
    y[[i, j]] += x;
    y[[i, k]] -= x;
    y[[j, i]] -= x;
    y[[j, k]] += x;
    y[[k, i]] += x;
    y[[k, j]] -= x;
}

fn calc_y(indeterminates: &HashMap<(usize, usize, usize), f32>, n: usize) -> M {
    let mut y = M::zeros((n, n));
    for (&triangle, x) in indeterminates {
        add_triangle_to_y(&mut y, triangle, *x);
    }
    y
}

struct TriangleRemover<'a> {
    y_mat: &'a mut M,
    n_mat: &'a mut M,
    triangles: &'a mut HashSet<(usize, usize, usize)>,
    indeterminates: &'a HashMap<(usize, usize, usize), f32>,
}

impl<'a> TriangleRemover<'a> {
    fn range_halving(&self, first: usize, last: usize) -> Vec<(usize, usize)> {
        match last - first {
            0 => vec![],
            1 => vec![(first, last)],
            n => {
                let mid = n / 2 + first;
                vec![(first, mid), (mid, last)]
            }
        }
    }

    fn graphic_remove(
        &mut self,
        (p_s, p_t): (usize, usize),
        (r_s, r_t): (usize, usize),
        (c_s, c_t): (usize, usize),
    ) -> bool {
        let p_len = p_t - p_s;
        let r_len = r_t - r_s;
        let c_len = c_t - c_s;
        let mut removed = false;

        if p_len == 1 && r_len == 1 && c_len == 1 {
            let triangle = (p_s, r_s, c_s);
            println!("Removing triangle {} {} {}", p_s, r_s, c_s);

            if let Some(x) = self.indeterminates.get(&triangle) {
                add_triangle_to_y(self.y_mat, triangle, -x);
                if let Ok(new_n) = self.y_mat.inv() {
                    *self.n_mat = new_n;
                    self.triangles.remove(&triangle);
                } else {
                    add_triangle_to_y(self.y_mat, triangle, *x);
                }
            }
        } else {
            let all_nodes = iproduct!(
                self.range_halving(p_s, p_t),
                self.range_halving(r_s, r_t),
                self.range_halving(c_s, c_t)
            );
            for (p, r, c) in all_nodes {
                let slice_removed = self.graphic_remove(p, r, c);
                removed = removed || slice_removed;
                if slice_removed {
                    *self.n_mat = self.y_mat.inv().unwrap();
                }
            }
        }
        removed
    }
}

fn augment_cactus(_full: &Graph, _cactus: &mut Graph) {}

pub fn cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut triangles = list_triangles(g);
    let indeterminates = triangles_to_indeterminates(&triangles);
    let mut y_mat = calc_y(&indeterminates, n);

    if let Ok(mut n_mat) = y_mat.inv() {
        let mut tr = TriangleRemover {
            y_mat: &mut y_mat,
            n_mat: &mut n_mat,
            triangles: &mut triangles,
            indeterminates: &indeterminates,
        };

        tr.graphic_remove((0, n), (0, n), (0, n));
    }

    // construct cactus with the remaining edges
    let mut maximum_cactus = triangles_to_cactus(n, &triangles);
    augment_cactus(g, &mut maximum_cactus);
    maximum_cactus
}
