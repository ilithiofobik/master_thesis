use crate::graphs::Graph;

// Based on Erdos–Gallai theorem.
fn is_graphical(d_seq: &[usize]) -> bool {
    if d_seq.iter().sum::<usize>() & 1 == 1 {
        return false;
    }

    let n = d_seq.len();
    let mut left_sum = 0;
    let mut d_sorted = d_seq.to_vec();
    d_sorted.sort_by(|a, b| b.cmp(a));
    let durfee_num = d_sorted
        .iter()
        .enumerate()
        .filter(|&(i, &di)| i <= di)
        .count();
    let max_deg = d_sorted[0];

    // sum from i to n
    let mut upper_sums = vec![0; n + 1];
    for i in (0..n).rev() {
        upper_sums[i] = upper_sums[i + 1] + d_sorted[i];
    }

    // define biggest i such that d_sorted[i] >= k for each k
    let mut biggest_i = vec![None; max_deg + 1];
    for (i, &di) in d_sorted.iter().enumerate() {
        biggest_i[di] = Some(i);
    }

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

    for (k, dk) in d_sorted.iter().enumerate().take(durfee_num) {
        left_sum += dk;
        let p = biggest_i.get(k + 2).unwrap_or(&Some(k)).unwrap_or(k).max(k);

        if left_sum > p * (k + 1) + upper_sums[p + 1] {
            return false;
        }
    }

    true
}

fn is_graphical_i_j(d_seq: &mut [usize], i: usize, j: usize) -> bool {
    d_seq[i] -= 1;
    d_seq[j] -= 1;
    let result = is_graphical(d_seq);
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
        .collect::<Vec<usize>>();
    indices_to_process.sort_by(|a, b| d[*b].cmp(&d[*a]));

    while let Some(i) = indices_to_process.pop() {
        let mut not_having_edge = indices_to_process.clone();

        while d[i] > 0 {
            let mut j_opt = None;
            let m = not_having_edge.len();

            // random choice without replacement
            // reject if the edge is not graphical
            for max_rand in (0..m).rev() {
                let r = fastrand::usize(0..=max_rand);
                not_having_edge.swap(r, max_rand);
                let j = not_having_edge[max_rand];

                if is_graphical_i_j(&mut d, i, j) {
                    j_opt = Some(j);
                    not_having_edge.swap(m - 1, max_rand);
                    not_having_edge.pop();
                    break;
                }
            }

            let j = j_opt.unwrap();
            graph.add_edge(i, j);

            d[i] -= 1;
            d[j] -= 1;

            let itp_index = indices_to_process.iter().position(|x| *x == j).unwrap();

            if d[j] == 0 {
                indices_to_process.remove(itp_index);
            } else {
                indices_to_process.select_nth_unstable_by(itp_index, |x, y| d[*y].cmp(&d[*x]));
            }
        }
    }

    Ok(graph)
}
