use crate::graphs::Graph;
use nalgebra::ComplexField;
use nalgebra::DMatrix;
use std::collections::HashSet;

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
    for triangle in triangles {
        cactus.add_edge(triangle.0, triangle.1);
        cactus.add_edge(triangle.1, triangle.2);
        cactus.add_edge(triangle.2, triangle.0);
    }
    cactus
}

fn graphic_remove(
    p: &mut [usize],
    r: &mut [usize],
    c: &mut [usize],
    g: &Graph,
    triangles: &mut Vec<(usize, usize, usize)>,
) {
    let p_len = p.len();
    let r_len = r.len();
    let c_len = c.len();

    if p_len == 1 && r_len == 1 && c_len == 1 {
        // process the triangle if it is in the list
    } else {
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
                    graphic_remove(
                        &mut p[p_s..p_t],
                        &mut r[r_s..r_t],
                        &mut c[c_s..c_t],
                        g,
                        triangles,
                    );
                    // TODO: compute N_S,S
                }
            }
        }

        graphic_remove(
            &mut p[..p_half],
            &mut r[..r_half],
            &mut c[..c_half],
            g,
            triangles,
        );
    }
}

pub fn cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut triangles = list_triangles(g);

    let mut p = g.vertices().collect::<Vec<usize>>();
    let mut r = p.clone();
    let mut c = p.clone();

    // TODO: construct Y and N

    graphic_remove(&mut p, &mut r, &mut c, g, &mut triangles);

    // construct cactus with the remaining edges
    triangles_to_cactus(n, &triangles)
}
