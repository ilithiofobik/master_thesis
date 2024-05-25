use std::collections::HashSet;

use crate::graphs::Graph;
use crate::mps_alg::*;
pub struct CactiApproximation {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VertexState {
    New,
    Active,
    Used,
}

fn add_triangle_to_graph(g: &mut Graph, u: usize, v: usize, w: usize) {
    g.add_edge(u, v);
    g.add_edge(v, w);
    g.add_edge(w, u);
}

fn connect_triangles(g: &Graph) -> (Graph, Vec<usize>) {
    let n = g.num_of_vertices();
    let mut cactus = Graph::empty(n);

    let mut new_vertices = (0..n).collect::<HashSet<usize>>();
    let mut active_vertices = HashSet::with_capacity(n);
    let mut vertex_states = vec![VertexState::New; n];
    let mut component = (0..n).collect::<Vec<usize>>();

    // connect triangles
    loop {
        if new_vertices.is_empty() {
            break;
        }

        if active_vertices.is_empty() {
            let x = *new_vertices.iter().next().unwrap();
            new_vertices.remove(&x);
            active_vertices.insert(x);
            vertex_states[x] = VertexState::Active;
        }

        let x = *active_vertices.iter().next().unwrap();
        let x_neighbors = g.neighbors(x).unwrap();

        for &y in x_neighbors {
            for &z in x_neighbors {
                if y == z {
                    continue;
                }

                if vertex_states[y] == VertexState::New && vertex_states[z] == VertexState::New {
                    add_triangle_to_graph(&mut cactus, x, y, z);
                    component[y] = component[x];
                    component[z] = component[x];
                    new_vertices.remove(&y);
                    new_vertices.remove(&z);
                    active_vertices.insert(y);
                    active_vertices.insert(z);
                    vertex_states[y] = VertexState::Active;
                    vertex_states[z] = VertexState::Active;
                }
            }
        }

        active_vertices.remove(&x);
        vertex_states[x] = VertexState::Used;
    }

    (cactus, component)
}

fn connect_cactus(g: &Graph, cactus: &mut Graph, c: &mut [usize]) {
    let n = cactus.num_of_vertices();
    let mut components = vec![vec![]; n];
    for i in 0..n {
        components[c[i]].push(i);
    }

    for u in 0..n {
        for &v in g.neighbors(u).unwrap() {
            if c[u] == c[v] {
                continue;
            }

            let u_component = components[c[u]].clone();

            for x in u_component {
                c[x] = c[v];
                components[c[v]].push(x);
            }

            components[c[u]].clear();

            cactus.add_edge(u, v);
        }
    }
}

impl MpsAlgorithm for CactiApproximation {
    fn maximum_planar_subgraph(g: &Graph) -> Graph {
        let (mut result, mut c) = connect_triangles(g);
        connect_cactus(g, &mut result, &mut c);
        result
    }
}
