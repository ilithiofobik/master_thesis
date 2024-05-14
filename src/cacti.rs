extern crate ndarray;
extern crate ndarray_linalg;

use crate::graphs::Graph;
use crate::triangle_listing::*;
use crate::union_find::*;
use fastrand;
use itertools::*;
use ndarray::Array2;

use ndarray_linalg::solve::Inverse;
use std::collections::HashMap;
use std::collections::HashSet;

type M = Array2<f32>;

/*
    In this module we suppose that the triangle (i, j, k) represents a pair of two edges: (i, j) and (i, k).
*/

pub fn purify_triangles(
    triangles: &HashSet<(usize, usize, usize)>,
) -> HashSet<(usize, usize, usize)> {
    let mut used_edges = HashSet::new();
    let mut purified = HashSet::new();
    for &(a, b, c) in triangles {
        if used_edges.contains(&(a, b))
            || used_edges.contains(&(a, c))
            || used_edges.contains(&(b, c))
        {
            continue;
        }
        used_edges.insert((a, b));
        used_edges.insert((a, c));
        used_edges.insert((b, c));
        purified.insert((a, b, c));
    }
    purified
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

trait CactusFinder {
    fn find_cactus(&self) -> Graph;
}

struct AlgebraicTriangleRemover<'a> {
    y_mat: &'a mut M,
    n_mat: &'a mut M,
    triangles: &'a mut HashSet<(usize, usize, usize)>,
    indeterminates: &'a HashMap<(usize, usize, usize), f32>,
}

impl<'a> AlgebraicTriangleRemover<'a> {
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
            if let Some(&x) = self.indeterminates.get(&triangle) {
                add_triangle_to_y(self.y_mat, triangle, -x);
                if let Ok(n_mat) = self.y_mat.inv() {
                    *self.n_mat = n_mat;
                    self.triangles.remove(&triangle);
                    println!("Removed triangle: {:?}", triangle);
                    println!("Y matrix: ");
                    print_matrix(self.y_mat);
                    removed = true;
                } else {
                    add_triangle_to_y(self.y_mat, triangle, x);
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

pub fn print_matrix(m: &M) {
    for i in 0..m.shape()[0] {
        for j in 0..m.shape()[1] {
            print!("{:.2} ", m[[i, j]]);
        }
        println!();
    }
}

fn augment_cactus(_full: &Graph, _cactus: &mut Graph) {}

pub fn cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut triangles = list_triangles(g);
    println!("Triangles: {:?}", triangles);
    println!("Purified triangles: {:?}", triangles);
    let indeterminates = triangles_to_indeterminates(&triangles);
    let mut y_mat = calc_y(&indeterminates, n);
    println!("Y matrix: ");
    print_matrix(&y_mat);

    if let Ok(mut n_mat) = y_mat.inv() {
        let mut tr = AlgebraicTriangleRemover {
            y_mat: &mut y_mat,
            n_mat: &mut n_mat,
            triangles: &mut triangles,
            indeterminates: &indeterminates,
        };

        tr.graphic_remove((0, n), (0, n), (0, n));
    }

    //triangles = purify_triangles(&triangles);

    // construct cactus with the remaining edges
    let mut maximum_cactus = triangles_to_cactus(n, &triangles);
    augment_cactus(g, &mut maximum_cactus);
    maximum_cactus
}

pub fn basic_cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut result = Graph::empty(n);

    let mut union_find = UnionFind::new(n);
    let triangle_lister = TriangleLister::new(g);

    // get maximal triangular cactus
    for (a, b, c) in triangle_lister {
        let pairs = [(a, b), (b, c), (c, a)];
        for (x, y) in pairs {
            if union_find.same_set(x, y) {
                continue;
            }
        }
        for (x, y) in pairs {
            union_find.union(x, y);
            result.add_edge(x, y);
        }
    }

    // extend to maximal structure
    for u in g.vertices() {
        for &v in g.neighbors(u).unwrap() {
            if union_find.same_set(u, v) {
                continue;
            }
            union_find.union(u, v);
            result.add_edge(u, v);
        }
    }

    result
}
