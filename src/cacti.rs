use crate::graphs::Graph;
use fastrand;
use ndarray::Array2;
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

pub fn list_triangles(g: &Graph) -> Vec<(usize, usize, usize)> {
    let n = g.num_of_vertices();
    let mut triangles = Vec::new();
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
                    triangles.push(triangle);
                }
            }
        }

        processed[v] = true;
    }

    triangles
}

fn triangles_to_cactus(n: usize, triangles: &[(usize, usize, usize)]) -> Graph {
    let mut cactus = Graph::empty(n);
    for &(a, b, c) in triangles {
        cactus.add_edge(a, b);
        cactus.add_edge(b, c);
        cactus.add_edge(c, a);
    }
    cactus
}

fn slices_concat3(p: &[usize], r: &[usize], c: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(p.len() + r.len() + c.len());
    result.extend_from_slice(p);
    result.extend_from_slice(r);
    result.extend_from_slice(c);
    result.sort();
    result.dedup();
    result
}

fn graphic_remove(
    y: &mut M,
    p: &mut [usize],
    r: &mut [usize],
    c: &mut [usize],
    g: &Graph,
    triangles: &mut Vec<(usize, usize, usize)>,
) -> bool {
    let p_len = p.len();
    let r_len = r.len();
    let c_len = c.len();
    let mut removed = false;

    if p_len == 1 && r_len == 1 && c_len == 1 {
        // process the triangle if it is in the list
        // if changed then removed = true
    } else {
        let _s = slices_concat3(p, r, c);
        let p_half = p_len / 2;
        let r_half = r_len / 2;
        let c_half = c_len / 2;

        for (p_s, p_t) in [(0, p_half), (p_half, p_len)] {
            // p slice is empty
            if p_s >= p_t {
                continue;
            }
            for (r_s, r_t) in [(0, r_half), (r_half, r_len)] {
                // r slice is empty
                if r_s >= r_t {
                    continue;
                }
                for (c_s, c_t) in [(0, c_half), (c_half, c_len)] {
                    // c slice is empty
                    if c_s >= c_t {
                        continue;
                    }
                    let slice_removed = graphic_remove(
                        y,
                        &mut p[p_s..p_t],
                        &mut r[r_s..r_t],
                        &mut c[c_s..c_t],
                        g,
                        triangles,
                    );
                    removed = removed || slice_removed;
                    if slice_removed {
                        // TODO: compute N_S,S
                    }
                }
            }
        }
    }
    removed
}

fn triangles_to_indeterminates(
    triangles: &[(usize, usize, usize)],
) -> HashMap<(usize, usize, usize), f32> {
    let mut indeterminates = HashMap::new();
    for triangle in triangles {
        indeterminates.insert(*triangle, 0.75 * fastrand::f32() + 0.25); // values between 0.25 and 1
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

pub fn cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut triangles = list_triangles(g);
    let indeterminates = triangles_to_indeterminates(&triangles);

    let mut p = g.vertices().collect::<Vec<usize>>();
    let mut r = p.clone();
    let mut c = p.clone();

    // TODO: construct Y and N
    let mut y = calc_y(&indeterminates, n);
    //let mut n_mat = y.

    graphic_remove(&mut y, &mut p, &mut r, &mut c, g, &mut triangles);

    // construct cactus with the remaining edges
    triangles_to_cactus(n, &triangles)
}
