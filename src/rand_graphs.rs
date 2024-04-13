use crate::graphs::Graph;
use fastrand::*;

fn is_graphical(d_seq: &[usize]) -> bool {
    let n = d_seq.len();
    let sum = d_seq.iter().sum::<usize>();

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

struct edge_prob {
    a: usize,
    b: usize,
    p: f64,
    alpha: f64,
}

/// Based on "A Sequential Algorithm for Generating Random Graphs"
/// https://web.stanford.edu/~saberi/sis2.pdf
///
/// Assume the every degree is at least 1.
pub fn random_with_degree_seq(d: &[usize]) -> Result<Graph, &'static str> {
    if !is_graphical(d) {
        return Err("The degree sequence is not graphical.");
    }

    let n = d.len();
    let m = d.iter().sum::<usize>() / 2;
    let mf = m as f64;

    let mut graph = Graph::empty(n);
    let dn = d.to_vec().clone();

    let mut possible_edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in i + 1..n {
            let alpha = 1.0 - ((d[i] * d[j]) as f64) / (4.0 * mf);
            possible_edges.push(edge_prob {
                a: i,
                b: j,
                p: (d[i] * d[j]) as f64 * alpha,
                alpha,
            });
        }
    }

    possible_edges = possible_edges
        .into_iter()
        .filter(|edge| edge.p > 0.0)
        .collect::<Vec<edge_prob>>();

    while !possible_edges.is_empty() {
        let sum = possible_edges.iter().map(|edge| edge.p).sum::<f64>();
        let r = fastrand::f64() * sum;
        let mut i = 0;
        let mut sum = 0.0;

        while sum < r {
            sum += possible_edges[i].p;
            i += 1;
        }

        i -= 1;
        let edge = possible_edges.remove(i);

        graph.add_edge(edge.a, edge.b);

        for edge in &mut possible_edges {
            if edge.a == edge.a || edge.b == edge.b {
                edge.p = 0.0;
            } else {
                let alpha = edge.alpha;
                edge.p = edge.p * (1.0 - alpha);
            }
        }

        possible_edges = possible_edges
            .into_iter()
            .filter(|edge| edge.p > 0.0)
            .collect::<Vec<edge_prob>>();
    }

    Ok(graph)
}
