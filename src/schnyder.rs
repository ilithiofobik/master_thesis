use std::collections::HashMap;

use crate::graphs::Graph;
use crate::mps_alg::*;
use good_lp::*;

/// Computes the maximum planar subgraph using the Schnyder algorithm.
///
/// # Arguments
/// * `g` - A reference to the input graph.
///
/// # Returns
/// * A `Graph` representing the maximum planar subgraph.
pub fn schnyder_mps(g: &Graph) -> Graph {
    let mut vars = ProblemVariables::new();

    let n = g.num_of_vertices();
    let m = g.num_of_edges();
    let edges = g.all_edges();

    let mut s = HashMap::with_capacity(m);
    let mut t = vec![vec![Vec::with_capacity(n); n]; 3];
    let mut a = vec![vec![HashMap::with_capacity(m); n]; 3];

    // s_e = 1 iff edge e is in the mps
    for &e in edges.iter() {
        let s_e = vars.add(variable().binary());
        s.insert(e, s_e);
    }

    // t_i,u,v has value 1 iff u <_i v
    for i in 0..3 {
        for u in 0..n {
            for _v in 0..n {
                let t_i_u_v = vars.add(variable().binary());
                t[i][u].push(t_i_u_v);
            }
        }
    }

    // a_i,v,e has value 1 iff for all u in e u <_i v
    for i in 0..3 {
        for v in 0..n {
            for &e in edges.iter() {
                let a_i_v_e = vars.add(variable().binary());
                a[i][v].insert(e, a_i_v_e);
            }
        }
    }

    // Define the objective function to maximize the number of selected edges
    let mut objective = Expression::from(0);
    for e in edges.iter() {
        objective += s[e];
    }

    let mut problem = vars.maximise(objective).using(highs);

    // Apply the Euler criterion
    if n > 2 {
        let mut edges_sum = Expression::from(0);
        for e in edges.iter() {
            edges_sum += s[e];
        }
        let bound = Expression::from((3 * n - 6) as i32);
        problem = problem.with(constraint!(edges_sum <= bound));
    }

    // Constraint 2a: s[e] <= a[0][v][e] + a[1][v][e] + a[2][v][e]
    for e in edges.iter() {
        for v in 0..n {
            if v == e.0 || v == e.1 {
                continue;
            }
            problem = problem.with(constraint!(s[e] <= a[0][v][e] + a[1][v][e] + a[2][v][e]));
        }
    }

    // Constraint 2b: a[i][v][e] <= t[i][u][v]
    for i in 0..3 {
        for e in edges.iter() {
            for u in [e.0, e.1] {
                for v in 0..n {
                    if v == e.0 || v == e.1 {
                        continue;
                    }
                    problem = problem.with(constraint!(a[i][v][e] <= t[i][u][v]));
                }
            }
        }
    }

    // Constraint 2c: t[0][u][v] + t[1][u][v] + t[2][u][v] <= 2.1
    for u in 0..n {
        for v in 0..n {
            if u == v {
                continue;
            }
            problem = problem.with(constraint!(t[0][u][v] + t[1][u][v] + t[2][u][v] <= 2.1));
        }
    }

    // Constraint 2d: t[i][u][v] + t[i][v][w] - 1 <= t[i][u][w]
    for i in 0..3 {
        for u in 0..n {
            for v in 0..n {
                if u == v {
                    continue;
                }
                for w in 0..n {
                    if u == w || v == w {
                        continue;
                    }
                    problem = problem.with(constraint!(t[i][u][v] + t[i][v][w] - 1 <= t[i][u][w]));
                }
            }
        }
    }

    // Constraint 2e: t[i][u][v] + t[i][v][u] == 1
    for i in 0..3 {
        for u in 0..n {
            for v in 0..n {
                if u == v {
                    continue;
                }
                problem = problem.with(constraint!(t[i][u][v] + t[i][v][u] == 1.0));
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

/// A struct representing the Schnyder MPS algorithm.
pub struct SchnyderMps {}

impl MpsAlgorithm for SchnyderMps {
    /// Computes the maximum planar subgraph using the Schnyder algorithm.
    ///
    /// # Arguments
    /// * `g` - A reference to the input graph.
    ///
    /// # Returns
    /// * A `Graph` representing the maximum planar subgraph.
    fn maximum_planar_subgraph(&self, g: &Graph) -> Graph {
        schnyder_mps(g)
    }

    /// Returns the name of the algorithm.
    ///
    /// # Returns
    /// * A string slice representing the name of the algorithm.
    fn name(&self) -> &'static str {
        "Schnyder"
    }
}
