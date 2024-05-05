use crate::cacti::cacti_approximation;
use crate::cacti::list_triangles;
use crate::graphs::Graph;

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
fn list_triangles_empty_test() {
    let graph = Graph::new();
    let triangles = list_triangles(&graph);

    assert_eq!(triangles.len(), 0);
}

#[test]
fn cacti_approximation_empty_test() {
    let graph = Graph::empty(0);
    let cacti = cacti_approximation(&graph);

    assert_eq!(cacti.num_of_vertices(), 0);
    assert_eq!(cacti.num_of_edges(), 0);
}

#[test]
fn cacti_approximation_k1_test() {
    let graph = Graph::complete(1);
    let cacti = cacti_approximation(&graph);

    assert_eq!(cacti.num_of_vertices(), 1);
    assert_eq!(cacti.num_of_edges(), 0);
}

#[test]
fn cacti_approximation_k6_test() {
    let graph = Graph::complete(6);
    let cacti = cacti_approximation(&graph);
    cacti.print_edges();

    assert_eq!(cacti.num_of_vertices(), 6);
    assert_eq!(cacti.num_of_edges(), 15);
    assert_eq!(cacti.neighbors(0), None);
    assert_eq!(cacti.neighbors(1), None);
    assert_eq!(cacti.neighbors(2), None);
    assert_eq!(cacti.neighbors(3), None);
    assert_eq!(cacti.neighbors(4), None);
    assert_eq!(cacti.neighbors(5), None);
}
