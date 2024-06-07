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

    for (k, &dk) in d_sorted.iter().enumerate() {
        if k > dk {
            break;
        }

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

fn pop_minimum(d: &[usize], indices: &mut Vec<usize>) -> Option<usize> {
    if indices.is_empty() {
        return None;
    }

    let n = indices.len();
    let mut min = d[indices[0]];
    let mut min_index = 0;

    for i in 1..n {
        if d[indices[i]] < min {
            min = d[indices[i]];
            min_index = i;
        }
    }

    indices.swap(min_index, n - 1);
    indices.pop()
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

    while let Some(i) = pop_minimum(&d, &mut indices_to_process) {
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
            }
        }
    }

    Ok(graph)
}

pub fn general_random_graph(
    num_of_vertices: usize,
    num_of_edges: usize,
) -> Result<Graph, &'static str> {
    if 2 * num_of_edges > num_of_vertices * num_of_vertices - num_of_vertices {
        return Err("The number of edges is too large.");
    }

    let mut graph = Graph::empty(num_of_vertices);

    let mut edges = (0..num_of_vertices)
        .flat_map(|from| (from + 1..num_of_vertices).map(move |to| (from, to)))
        .collect::<Vec<(usize, usize)>>();

    fastrand::shuffle(&mut edges);

    (0..num_of_edges).map(|i| edges[i]).for_each(|(from, to)| {
        graph.add_edge(from, to);
    });

    Ok(graph)
}
