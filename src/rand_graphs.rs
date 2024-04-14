use crate::graphs::Graph;
use std::collections::HashMap;

const MAX_ITER: usize = 1000;

fn is_graphical(d_seq: &[usize]) -> bool {
    // sort non-decreasing
    let mut d_seq = d_seq.to_vec();
    d_seq.sort_by(|a, b| a.cmp(b));

    while let Some(d) = d_seq.pop() {
        // all degrees are zero
        if d == 0 {
            return true;
        }

        let n = d_seq.len();

        // not enough degrees left
        if d > n {
            return false;
        }

        for i in n - d..n {
            if d_seq[i] == 0 {
                // cannot connect vertices
                return false;
            }

            d_seq[i] -= 1;
        }

        d_seq.sort_by(|a, b| a.cmp(b));
    }

    true
}

fn calculate_alpha_ppbs(
    d: &[usize],
) -> (HashMap<(usize, usize), f64>, HashMap<(usize, usize), f64>) {
    let n = d.len();
    let m = d.iter().sum::<usize>() / 2;
    let mf = m as f64;
    let inv_mf = mf.recip();

    let edges_num = n * (n - 1) / 2;
    let mut ppbs = HashMap::with_capacity(edges_num);
    let mut alpha = HashMap::with_capacity(edges_num);

    // initialize alpha and possible edges
    // use only those with positive probability
    for i in 0..n {
        if d[i] == 0 {
            continue;
        }

        let di = d[i] as f64;

        for j in i + 1..n {
            if d[j] == 0 {
                continue;
            }

            let dj = d[j] as f64;
            let a = 1.0 - (0.25 * di * dj * inv_mf);
            let p = di * dj * a;

            alpha.insert((i, j), a);
            ppbs.insert((i, j), p);
        }
    }

    (alpha, ppbs)
}

fn calc_p(dn: &[usize], alpha: &HashMap<(usize, usize), f64>, a: usize, b: usize) -> f64 {
    let da = dn[a] as f64;
    let db = dn[b] as f64;

    da * db * alpha.get(&(a, b)).unwrap()
}

fn remove_vertex(ppbs: &mut HashMap<(usize, usize), f64>, n: usize, v: usize) {
    for x in 0..v {
        ppbs.remove(&(x, v));
    }
    for y in v + 1..n {
        ppbs.remove(&(v, y));
    }
}

fn update_vertex(
    dn: &[usize],
    ppbs: &mut HashMap<(usize, usize), f64>,
    alpha: &HashMap<(usize, usize), f64>,
    n: usize,
    v: usize,
) {
    for x in 0..v {
        let p = calc_p(dn, alpha, x, v);
        if p > 0.0 && ppbs.contains_key(&(x, v)) {
            ppbs.insert((x, v), p);
        }
    }
    for y in v + 1..n {
        let p = calc_p(dn, alpha, v, y);
        if p > 0.0 && ppbs.contains_key(&(v, y)) {
            ppbs.insert((v, y), p);
        }
    }
}

/// Based on "A Sequential Algorithm for Generating Random Graphs"
/// https://web.stanford.edu/~saberi/sis2.pdf
fn try_generate_with_degree_seq(
    d: &[usize],
    alpha: &HashMap<(usize, usize), f64>,
    ppbs: &HashMap<(usize, usize), f64>,
) -> Result<Graph, &'static str> {
    let n = d.len();
    let m = d.iter().sum::<usize>() / 2;

    let mut graph = Graph::empty(n);
    let mut dn = d.to_vec();
    let mut ppbs = ppbs.clone();

    while !ppbs.is_empty() {
        let total_sum = ppbs.values().sum::<f64>();
        let r = fastrand::f64() * total_sum;
        let mut i = 0;
        let mut sum = 0.0;

        for (_, &p) in &ppbs {
            sum += p;
            if sum >= r {
                break;
            }
            i += 1;
        }

        let (a, b) = *ppbs.keys().nth(i).unwrap();
        let added_edge = graph.add_edge(a, b);
        assert!(added_edge);

        // update possible edges
        dn[a] -= 1;
        dn[b] -= 1;
        ppbs.remove(&(a, b));

        if dn[a] == 0 {
            remove_vertex(&mut ppbs, n, a);
        } else {
            update_vertex(&dn, &mut ppbs, alpha, n, a);
        }

        if dn[b] == 0 {
            remove_vertex(&mut ppbs, n, b);
        } else {
            update_vertex(&dn, &mut ppbs, alpha, n, b);
        }

        // update and remove zero values
        // ppbs = ppbs
        //     .into_iter()
        //     .map(|((x, y), _)| ((x, y), calc_p(&dn, alpha, x, y)))
        //     .filter(|(_, p)| *p > 0.0)
        //     .collect::<HashMap<(usize, usize), f64>>();
    }

    if graph.num_of_edges() != m {
        println!("Num of edges: {}", graph.num_of_edges());
        eprintln!("Num of edges: {}", graph.num_of_edges());
        return Err("Failed to generate a graph. Not enough edges.");
    }

    Ok(graph)
}

pub fn random_with_degree_seq(d: &[usize]) -> Result<(Graph, usize), &'static str> {
    if !is_graphical(d) {
        return Err("The degree sequence is not graphical.");
    }

    let (alpha, ppbs) = calculate_alpha_ppbs(d);
    let m = d.iter().sum::<usize>() / 2;
    let d_max = d.into_iter().max().unwrap_or(&0);
    let max_iter = 100 * m * d_max;
    let mut try_num = 0;

    for _ in 0..max_iter {
        try_num += 1;
        let graph = try_generate_with_degree_seq(d, &alpha, &ppbs);
        if let Ok(g) = graph {
            return Ok((g, try_num));
        }
    }

    Err("Failed to generate a graph. Max iterations exceeded.")
}
