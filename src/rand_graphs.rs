use crate::graphs::Graph;
use std::collections::HashSet;

// Based on Erdős–Gallai theorem.
fn is_graphical(d_seq: &[usize]) -> bool {
    let sum = d_seq.iter().sum::<usize>();
    let n = d_seq.len();

    if sum % 2 != 0 {
        return false;
    }

    let mut left_sum = 0;
    let mut d_sorted = d_seq.to_vec();
    d_sorted.sort_by(|a, b| b.cmp(a));

    // sum from i to n
    let mut upper_sums = vec![0; n + 1];
    for i in (0..n).rev() {
        upper_sums[i] = upper_sums[i + 1] + d_sorted[i];
    }

    // define biggest i such that d_sorted[i] >= k for each k
    let mut biggest_i = vec![None; n + 2];
    for i in 0..n {
        biggest_i[d_sorted[i]] = Some(i);
    }
    let max_deg = d_sorted[0];
    let mut current_max = biggest_i[max_deg].unwrap();
    for k in (0..=max_deg).rev() {
        match biggest_i[k] {
            Some(j) => {
                current_max = j;
            }
            None => {
                biggest_i[k] = Some(current_max);
            }
        }
    }

    for k in 0..n {
        left_sum += d_sorted[k];
        let p = biggest_i[k + 2].unwrap_or(k).max(k);

        if left_sum > p * (k + 1) + upper_sums[p + 1] {
            return false;
        }
    }

    true
}

fn is_graphical_i_j(d_seq: &mut Vec<usize>, i: usize, j: usize) -> bool {
    d_seq[i] -= 1;
    d_seq[j] -= 1;
    let result = is_graphical(&d_seq);
    d_seq[i] += 1;
    d_seq[j] += 1;
    result
}

fn get_rand_neighbour(ppb_sum: usize, graphical_candidates: &[&usize], d: &[usize]) -> usize {
    let r = fastrand::usize(0..ppb_sum);
    let mut sum = 0;
    let mut j_idx = 0;

    while sum <= r {
        sum += d[*graphical_candidates[j_idx]];
        j_idx += 1;
    }
    j_idx -= 1;

    *graphical_candidates[j_idx]
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

    while let Some(i) = indices_to_process.iter().next().copied() {
        indices_to_process.remove(&i);
        let mut not_having_edge = indices_to_process.clone();

        while d[i] > 0 {
            let mut ppb_sum = 0;
            let graphical_candidates = not_having_edge
                .iter()
                .filter(|&j| {
                    if is_graphical_i_j(&mut d, i, *j) {
                        ppb_sum += d[*j];
                        return true;
                    }
                    false
                })
                .collect::<Vec<&usize>>();

            let j = get_rand_neighbour(ppb_sum, &graphical_candidates, &d);
            graph.add_edge(i, j);

            d[i] -= 1;
            d[j] -= 1;
            not_having_edge.remove(&j);

            if d[j] == 0 {
                indices_to_process.remove(&j);
            }
        }
    }

    Ok(graph)
}
