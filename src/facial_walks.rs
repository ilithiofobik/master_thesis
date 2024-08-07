use crate::graphs::Graph;
use crate::mps_alg::*;
use good_lp::*;
use std::collections::HashMap;

/// Sums the variables associated with the given edges.
///
/// # Arguments
/// * `edges` - A vector of edges represented as tuples of vertex indices.
/// * `s` - A hashmap mapping edges to their corresponding variables.
///
/// # Returns
/// * An expression representing the sum of the variables associated with the edges.
fn sum_s_over_e(edges: &Vec<(usize, usize)>, s: &HashMap<(usize, usize), Variable>) -> Expression {
    edges.iter().fold(Expression::from(0), |acc, e| acc + s[e])
}

/// Sums the given variables.
///
/// # Arguments
/// * `x` - A vector of variables.
///
/// # Returns
/// * An expression representing the sum of the variables.
fn sum_x_over_i(x: &Vec<Variable>) -> Expression {
    x.iter().fold(Expression::from(0), |acc, x_i| acc + x_i)
}

/// Sums the variables associated with the given faces and arcs.
///
/// # Arguments
/// * `c` - A vector of hashmaps mapping arcs to their corresponding variables for each face.
/// * `faces` - A slice of face indices.
/// * `arcs` - A slice of arcs represented as tuples of vertex indices.
///
/// # Returns
/// * An expression representing the sum of the variables associated with the faces and arcs.
fn sum_c_over_faces_arcs(
    c: &Vec<HashMap<(usize, usize), Variable>>,
    faces: &[usize],
    arcs: &[(usize, usize)],
) -> Expression {
    let mut result = Expression::from(0);
    for i in faces.iter() {
        for a in arcs.iter() {
            result += c[*i][a];
        }
    }
    result
}

/// Sums the variables associated with the given vertex and its neighbors.
///
/// # Arguments
/// * `p` - A hashmap mapping vertex triples to their corresponding variables.
/// * `v` - A vertex index.
/// * `us` - A slice of neighbor vertex indices.
/// * `ws` - A slice of neighbor vertex indices.
///
/// # Returns
/// * An expression representing the sum of the variables associated with the vertex and its neighbors.
fn sum_p_over_vertices(
    p: &HashMap<(usize, usize, usize), Variable>,
    v: usize,
    us: &[usize],
    ws: &[usize],
) -> Expression {
    let mut result = Expression::from(0);
    for &u in us.iter() {
        for &w in ws.iter() {
            if u == w {
                continue;
            }
            result += p[&(v, u, w)];
        }
    }
    result
}

/// Computes the double power set of the given elements.
///
/// # Arguments
/// * `elements` - A slice of elements.
///
/// # Returns
/// * A vector of tuples, where each tuple contains two subsets of the elements.
fn double_power_set(elements: &[usize]) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut result = Vec::new();
    let n = elements.len();
    let num_of_partitions = (1 << n) - 1;
    for i in 1..num_of_partitions {
        let mut subset0 = Vec::new();
        let mut subset1 = Vec::new();
        for j in 0..n {
            if i & (1 << j) == 0 {
                subset0.push(elements[j]);
            } else {
                subset1.push(elements[j]);
            }
        }
        result.push((subset0, subset1));
    }
    result
}

/// Converts an arc to a standardized edge representation (with the smaller vertex index first).
///
/// # Arguments
/// * `arc` - A tuple representing an arc.
///
/// # Returns
/// * A tuple representing the edge with the smaller vertex index first.
fn edge(arc: (usize, usize)) -> (usize, usize) {
    if arc.0 < arc.1 {
        arc
    } else {
        (arc.1, arc.0)
    }
}

/// Computes the maximum planar subgraph using facial walks.
///
/// # Arguments
/// * `g` - The input graph.
///
/// # Returns
/// * The maximum planar subgraph of the input graph.
pub fn facial_walks_mps(g: &Graph) -> Graph {
    let mut vars = ProblemVariables::new();

    let n = g.num_of_vertices();
    let m = g.num_of_edges();
    let f_max = 2 + m - n;

    let edges = g.all_edges();
    let arcs = g.all_arcs();
    let all_faces = (0..f_max).collect::<Vec<usize>>();

    let mut s = HashMap::with_capacity(m);
    let mut x = Vec::with_capacity(f_max);
    let mut c = vec![HashMap::with_capacity(2 * m); f_max];
    let mut p = HashMap::with_capacity(4 * m);

    // s_e = 1 iff edge e is in the mps
    for &e in edges.iter() {
        let s_e = vars.add(variable().binary());
        s.insert(e, s_e);
    }

    // x_i has value 1 iff face i exists
    for _i in all_faces.iter() {
        let x_i = vars.add(variable().binary());
        x.push(x_i);
    }

    // c_i,a has value 1 iff arc a bounds face i: traversing i in clockwise order visits e(a) in the orientation of a
    for &a in arcs.iter() {
        for i in 0..f_max {
            let c_i_a = vars.add(variable().binary());
            c[i].insert(a, c_i_a);
        }
    }

    // p_v,u,w has value 1 iff w is the direct successor of u in the cyclic order around v
    for v in 0..n {
        let neighbors = g.neighbors(v).unwrap();
        for u in neighbors.iter() {
            for w in neighbors.iter() {
                if u == w {
                    continue;
                }
                let p_v_u_w = vars.add(variable().binary());
                p.insert((v, *u, *w), p_v_u_w);
            }
        }
    }

    let objective = sum_s_over_e(&edges, &s);
    let mut problem = vars.maximise(objective).using(highs);

    // Euler criterion
    if n > 2 {
        let mut edges_sum = Expression::from(0);
        for e in edges.iter() {
            edges_sum += s[e];
        }
        let bound = Expression::from((3 * n - 6) as i32);
        problem = problem.with(constraint!(edges_sum <= bound));
    }

    // 1a
    {
        let x_f = sum_x_over_i(&x);
        let sum_of_s = sum_s_over_e(&edges, &s);

        problem = problem.with(constraint!(n as i32 + x_f == 2 + sum_of_s));
    }

    // 1c
    for i in 0..f_max - 1 {
        problem = problem.with(constraint!(x[i] >= x[i + 1]));
    }

    // 1d
    for i in 0..f_max {
        let arcs_sum = sum_c_over_faces_arcs(&c, &[i], &arcs);
        problem = problem.with(constraint!(3 * x[i] <= arcs_sum));
    }

    // 1e
    for i in 0..f_max {
        for &a in arcs.iter() {
            problem = problem.with(constraint!(c[i][&a] <= x[i]));
        }
    }

    // 1f
    for &a in arcs.iter() {
        problem = problem.with(constraint!(
            sum_c_over_faces_arcs(&c, &all_faces, &[a]) == s[&edge(a)]
        ));
    }

    // 1g
    for v in 0..n {
        let neighbors = g.neighbors(v).unwrap();
        let in_arcs = neighbors
            .iter()
            .cloned()
            .map(|u| (u, v))
            .collect::<Vec<_>>();
        let out_arcs = neighbors
            .iter()
            .cloned()
            .map(|u| (v, u))
            .collect::<Vec<_>>();
        for &i in all_faces.iter() {
            let in_c = sum_c_over_faces_arcs(&c, &[i], &in_arcs);
            let out_c = sum_c_over_faces_arcs(&c, &[i], &out_arcs);
            problem = problem.with(constraint!(in_c == out_c));
        }
    }

    // 1h, 1i
    for v in 0..n {
        let neighbors = g.neighbors(v).unwrap();
        for &u in neighbors.iter() {
            for &w in neighbors.iter() {
                if u == w {
                    continue;
                }
                for &i in all_faces.iter() {
                    problem = problem.with(constraint!(
                        c[i][&(v, w)] >= c[i][&(u, v)] + p[&(v, u, w)] - 1
                    ));
                    problem = problem.with(constraint!(
                        c[i][&(u, v)] >= c[i][&(v, w)] + p[&(v, u, w)] - 1
                    ));
                }
            }
        }
    }

    // 1j, 1k
    for v in 0..n {
        let neighbors = g
            .neighbors(v)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<usize>>();

        for &u in neighbors.iter() {
            let pvun = sum_p_over_vertices(&p, v, &[u], &neighbors);
            let pvnu = sum_p_over_vertices(&p, v, &neighbors, &[u]);
            let edge = edge((v, u));
            problem = problem.with(constraint!(pvun == s[&edge]));
            problem = problem.with(constraint!(pvnu == s[&edge]));
        }
    }

    // 1l
    for v in 0..n {
        let neighbors = g
            .neighbors(v)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<usize>>();
        let double_power_set = double_power_set(&neighbors);
        for (u, nu) in double_power_set.iter() {
            let pvun = sum_p_over_vertices(&p, v, &u, &nu);
            for &u_i in u {
                for &nu_i in nu {
                    problem = problem.with(constraint!(
                        pvun.clone() >= s[&edge((v, u_i))] + s[&edge((v, nu_i))] - 1
                    ));
                }
            }
        }
    }

    let solution = problem.solve().unwrap();

    let mut mps = Graph::empty(n);

    for &e in edges.iter() {
        if solution.value(s[&e]) >= 0.5 {
            mps.add_edge(e.0, e.1);
        }
    }

    mps
}

/// A struct representing the facial walks MPS algorithm.
pub struct FacialWalksMps {}

impl MpsAlgorithm for FacialWalksMps {
    /// Computes the maximum planar subgraph of the given graph.
    ///
    /// # Arguments
    /// * `g` - The input graph.
    ///
    /// # Returns
    /// * The maximum planar subgraph of the input graph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        facial_walks_mps(g)
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "FacialWalks"
    }
}
