use crate::rand_graphs::*;

fn bliztstein_test(d: &[usize]) {
    let graph = bliztstein_generation(d).unwrap();
    assert_eq!(graph.num_of_edges(), d.iter().sum::<usize>() / 2);

    for (u, &du) in d.iter().enumerate() {
        match graph.neighbors(u) {
            Some(neighbors) => {
                assert_eq!(neighbors.len(), du);
            }
            None => panic!("Vertex {} not found", u),
        }
    }
}

fn general_random_test(n: usize, k: usize) {
    let graph = general_random_graph(n, k);

    match graph {
        Ok(graph) => {
            assert_eq!(graph.num_of_vertices(), n);
            assert_eq!(graph.num_of_edges(), k);
        }
        Err(_) => {
            assert!(2 * k > n * n - n);
        }
    }
}

#[test]
fn bliztstein_generation_regular_test() {
    let d = vec![10; 1000];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_test() {
    let d = vec![
        7, 8, 5, 1, 1, 2, 8, 10, 4, 2, 4, 5, 3, 6, 7, 3, 2, 7, 6, 1, 2, 9, 6, 1, 3, 4, 6, 3, 3, 3,
        2, 4, 4,
    ];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_complete_test() {
    let d = vec![99; 100];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_non_graphical_test() {
    let d = vec![4, 3, 2, 1, 0];
    assert!(bliztstein_generation(&d).is_err());
}

#[test]
fn general_random_0_vertices_0_edges_test() {
    general_random_test(0, 0)
}

#[test]
fn general_random_100_vertices_0_edges_test() {
    general_random_test(100, 0)
}

#[test]
fn general_random_0_vertices_100_edges_test() {
    general_random_test(0, 100)
}

#[test]
fn general_random_100_vertices_100_edges_test() {
    general_random_test(100, 100)
}
