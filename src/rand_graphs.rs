use crate::graphs::Graph;
use std::collections::HashSet;

fn is_graphical(d_seq: &[usize]) -> bool {
    // sort non-decreasing
    let mut d_seq = d_seq
        .iter()
        .filter_map(|n| if *n > 0 { Some(*n) } else { None })
        .collect::<Vec<usize>>();
    d_seq.sort();

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

        for di in d_seq.iter_mut().skip(n - d) {
            if *di == 0 {
                // cannot connect vertices
                return false;
            }

            *di -= 1;
        }

        d_seq.sort();
    }

    true
}

fn is_graphical_i_j(d_seq: &[usize], i: usize, j: usize) -> bool {
    let mut d_seq = d_seq.to_vec();
    d_seq[i] -= 1;
    d_seq[j] -= 1;
    is_graphical(&d_seq)
}

// Based on "A Sequential Importance Sampling Algorithm for Generating Random Graphs with Prescribed Degrees"
// by J. Bliztstein and P. Diaconis
pub fn bliztstein_generation(d_in: &[usize]) -> Result<Graph, &'static str> {
    if !is_graphical(d_in) {
        return Err("The degree sequence is not graphical.");
    }

    let n = d_in.len();
    let mut graph = Graph::empty(n);
    let mut d = d_in.to_vec();
    let mut indices_to_process = d_in
        .iter()
        .enumerate()
        .filter_map(|(i, &di)| if di > 0 { Some(i) } else { None })
        .collect::<HashSet<usize>>();

    while !indices_to_process.is_empty() {
        let i = *indices_to_process.iter().next().unwrap();
        indices_to_process.remove(&i);

        while d[i] > 0 {
            let mut ppb_sum = 0;
            let candidates = indices_to_process
                .iter()
                .filter(|&j| {
                    let result = !graph.has_edge(i, *j) && is_graphical_i_j(&d, i, *j);
                    if result {
                        ppb_sum += d[*j];
                    }
                    result
                })
                .collect::<Vec<_>>();

            let r = fastrand::usize(0..ppb_sum);
            let mut sum = 0;
            let mut j_idx = 0;

            while sum <= r {
                sum += d[*candidates[j_idx]];
                j_idx += 1;
            }

            j_idx -= 1;
            let j = *candidates[j_idx];
            graph.add_edge(i, j);

            d[i] -= 1;
            d[j] -= 1;

            if d[j] == 0 {
                indices_to_process.remove(&j);
            }
        }
    }

    Ok(graph)
}
