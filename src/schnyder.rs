use std::collections::HashMap;

use crate::graphs::Graph;
use good_lp::*;

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
            for v in 0..n {
                let t_i_u_v = vars.add(variable().binary());
                t[i][u].push(t_i_u_v);
            }
        }
    }

    // a_i,e,v has value 1 iff for all u in e u <_i v
    for i in 0..3 {
        for v in 0..n {
            for &e in edges.iter() {
                let a_i_v_e = vars.add(variable().binary());
                a[i][v].insert(e, a_i_v_e);
            }
        }
    }

    let mut objective = Expression::from(0);
    for e in edges.iter() {
        objective += s[e];
    }

    let model = vars.maximise(objective).using(highs).solve().unwrap();

    let mut mps = Graph::empty(n);

    for &e in edges.iter() {
        if model.value(s[&e]) == 1.0 {
            mps.add_edge(e.0, e.1);
        }
    }

    mps
}
