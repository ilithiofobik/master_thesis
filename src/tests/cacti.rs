use crate::cacti::cacti_approximation;
use crate::cacti::list_triangles;
use crate::graphs::Graph;

fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    if k == 0 {
        return 1;
    }

    (n * binomial(n - 1, k - 1)) / k
}

#[test]
fn list_triangles_test() {
    let mut graph = Graph::complete(5);
    graph.remove_edge(0, 1);
    graph.remove_edge(3, 4);

    let mut triangles = list_triangles(&graph)
        .into_iter()
        .collect::<Vec<(usize, usize, usize)>>();
    triangles.sort();

    assert_eq!(triangles.len(), 4);
    assert_eq!(triangles[0], (0, 2, 3));
    assert_eq!(triangles[1], (0, 2, 4));
    assert_eq!(triangles[2], (1, 2, 3));
    assert_eq!(triangles[3], (1, 2, 4));
}

#[test]
fn list_triangles_complete_test() {
    for n in 0..10 {
        println!("list_triangles_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let triangles = list_triangles(&graph);

        assert_eq!(triangles.len(), binomial(n, 3));
    }
}

//#[test]
fn _cacti_approximation_complete_test() {
    for n in 0..10 {
        println!("cacti_approximation_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let cacti = cacti_approximation(&graph);

        assert_eq!(cacti.num_of_vertices(), n);
        assert_eq!(cacti.num_of_edges(), 3 * (n / 3));
    }
}
