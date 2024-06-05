use std::collections::HashMap;

use crate::graphs::Graph;
use good_lp::*;

fn swap(e: (usize, usize)) -> (usize, usize) {
    (e.1, e.0)
}

fn edge(arc: (usize, usize)) -> (usize, usize) {
    if arc.0 < arc.1 {
        arc
    } else {
        swap(arc)
    }
}

pub fn lr_coloring_mps(g: &Graph) -> Graph {
    let mut vars = ProblemVariables::new();

    let n = g.num_of_vertices();
    let m = g.num_of_edges();
    let edges = g.all_edges();
    let mut arcs = Vec::with_capacity(2 * m);

    for &e in edges.iter() {
        arcs.push(e);
        arcs.push(swap(e));
    }

    let mut s = HashMap::with_capacity(m);
    let mut t = HashMap::with_capacity(2 * m);
    let mut r = HashMap::with_capacity(m);
    let mut l = vec![Vec::with_capacity(n); n];

    // s_e = 1 iff edge e is in the mps
    // r_e = 1 iff edge e is colored red
    for &e in edges.iter() {
        let s_e = vars.add(variable().binary());
        let r_e = vars.add(variable().binary());
        s.insert(e, s_e);
        r.insert(e, r_e);
    }

    // t_e has value 1 iff arc d is in the Tremaux tree T
    for &d in arcs.iter() {
        let t_d = vars.add(variable().binary());
        t.insert(d, t_d);
    }

    // l_u,v has value 1 iff node u lies on the path from the root to node v in T
    for u in 0..n {
        for _v in 0..n {
            let l_u_v = vars.add(variable().binary());
            l[u].push(l_u_v);
        }
    }

    let mut objective = Expression::from(0);
    for e in edges.iter() {
        objective += s[e];
    }

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

    // 3a
    let mut arcs_sum = Expression::from(0);
    for d in arcs.iter() {
        arcs_sum += t[d];
    }
    let arcs_bound = Expression::from((n - 1) as i32);
    problem = problem.with(constraint!(arcs_sum == arcs_bound));

    // 3b, 3c
    for d in arcs.iter() {
        problem = problem.with(constraint!(t[d] <= s[&edge(*d)]));
        problem = problem.with(constraint!(t[d] <= l[d.0][d.1]));
    }

    // 3d
    for u in 0..n {
        let neighbors = g.neighbors(u).unwrap();

        for (i, &v) in neighbors.iter().enumerate() {
            for &w in neighbors.iter().skip(i) {
                problem = problem.with(constraint!(
                    l[v][w] + l[w][v] + t[&(u, v)] + t[&(u, w)] <= 2
                ));
            }
        }
    }

    // 3 e,f
    for u in 0..n {
        for v in 0..n {
            if u == v {
                continue;
            }

            for w in 0..n {
                if u == w || v == w {
                    continue;
                }

                problem = problem.with(constraint!(l[u][w] + l[v][w] <= 1 + l[u][v] + l[v][u]));
                problem = problem.with(constraint!(l[u][v] + l[v][w] <= l[u][w] + 1));
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
