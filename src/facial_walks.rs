use std::collections::HashMap;

use crate::graphs::Graph;
use good_lp::*;

fn sum_s_over_e(edges: &Vec<(usize, usize)>, s: &HashMap<(usize, usize), Variable>) -> Expression {
    edges.iter().fold(Expression::from(0), |acc, e| acc + s[e])
}

fn sum_x_over_i(x: &Vec<Variable>) -> Expression {
    x.iter().fold(Expression::from(0), |acc, x_i| acc + x_i)
}

pub fn facial_walks_mps(g: &Graph) -> Graph {
    let mut vars = ProblemVariables::new();

    let n = g.num_of_vertices();
    let m = g.num_of_edges();
    let f_max = 2 + m - n;
    let edges = g.all_edges();
    let arcs = g.all_arcs();

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
    for i in 0..f_max {
        let x_i = vars.add(variable().binary());
        x.push(x_i);
    }

    // c_i,a has value 1 iff arc a bounds face i: traversing i in clockwise order visits e(a) in the orientation of a
    for a in arcs {
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
                p.insert((v, u, w), p_v_u_w);
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
        let n = Expression::from(n as i32);
        let x_f = sum_x_over_i(&x);
        let two = Expression::from(2);
        let sum_of_s = sum_s_over_e(&edges, &s);

        problem = problem.with(constraint!(n + x_f == two + sum_of_s));
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
