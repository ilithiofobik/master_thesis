use crate::graphs::Graph;

/// Determines if a degree sequence is graphical based on the Erdos–Gallai theorem.
///
/// # Arguments
/// * `d_seq` - A slice of integers representing the degree sequence.
///
/// # Returns
/// * `true` if the degree sequence is graphical, `false` otherwise.
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

/// Checks if removing an edge between vertices i and j keeps the degree sequence graphical.
///
/// # Arguments
/// * `d_seq` - A mutable slice of integers representing the degree sequence.
/// * `i` - The index of the first vertex.
/// * `j` - The index of the second vertex.
///
/// # Returns
/// * `true` if the degree sequence remains graphical after removing the edge, `false` otherwise.
fn is_graphical_i_j(d_seq: &mut [usize], i: usize, j: usize) -> bool {
    d_seq[i] -= 1;
    d_seq[j] -= 1;
    let result = is_graphical(d_seq);
    d_seq[i] += 1;
    d_seq[j] += 1;
    result
}

/// Generates a random graph with a given degree sequence using the Blitzstein-Diaconis algorithm.
///
/// # Arguments
/// * `d_in` - A slice of integers representing the degree sequence.
///
/// # Returns
/// * `Ok(Graph)` if the degree sequence is graphical and a graph is generated, `Err(&'static str)` otherwise.
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

/// Generates a general random graph with a specified number of vertices and edges.
///
/// # Arguments
/// * `num_of_vertices` - The number of vertices in the graph.
/// * `num_of_edges` - The number of edges in the graph.
///
/// # Returns
/// * `Ok(Graph)` if the graph is successfully generated, `Err(&'static str)` if the number of edges is too large.
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

/// Generates a random regular graph with a specified number of vertices and degree.
///
/// # Arguments
/// * `n` - The number of vertices in the graph.
/// * `d` - The degree of each vertex.
///
/// # Returns
/// * `Ok(Graph)` if the graph is successfully generated, `Err(&'static str)` if the degree sequence is not graphical.
pub fn random_regular_graph(n: usize, d: usize) -> Result<Graph, &'static str> {
    let d_seq = vec![d; n];
    bliztstein_generation(&d_seq)
}

/// Generates a Pareto-distributed value.
///
/// # Arguments
/// * `n` - The number of values.
/// * `alpha` - The Pareto distribution parameter.
///
/// # Returns
/// * A `usize` value generated according to the Pareto distribution.
fn pareto_value(n: usize, alpha: f64) -> usize {
    let beta = 1.0 - (n as f64 - 1.0).powf(-alpha);
    let y = fastrand::f64();
    (1.0 - beta * y).powf(-1.0 / alpha).round() as usize
}

/// Generates a Pareto-distributed degree sequence.
///
/// # Arguments
/// * `n` - The number of vertices.
/// * `alpha` - The Pareto distribution parameter.
///
/// # Returns
/// * A vector of `usize` values representing the degree sequence.
fn generate_pareto_sequence(n: usize, alpha: f64) -> Vec<usize> {
    let mut result = vec![0; n];
    let mut even: usize = 0;

    for i in 0..n {
        let v = pareto_value(n - 1, alpha);
        even = even ^ (v & 1);
        result[i] = v;
    }

    let mut change_index = 0;

    while even == 1 || !is_graphical(&result) {
        let v = pareto_value(n - 1, alpha);
        even = even ^ (v & 1);
        result[change_index] = v;
        change_index = (change_index + 1) % n;
    }

    result
}

/// Generates a random graph with a Pareto degree distribution.
///
/// # Arguments
/// * `n` - The number of vertices in the graph.
/// * `alpha` - The Pareto distribution parameter.
///
/// # Returns
/// * `Ok(Graph)` if the graph is successfully generated, `Err(&'static str)` if the degree sequence is not graphical.
pub fn random_pareto_graph(n: usize, alpha: f64) -> Result<Graph, &'static str> {
    let d_seq = generate_pareto_sequence(n, alpha);
    bliztstein_generation(&d_seq)
}

// TODO: przerobić żeby brał najmniejsze degree zawsze
