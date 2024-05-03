use crate::cacti::cacti_approximation;
use crate::cacti::list_triangles;
use crate::graphs::Graph;

#[test]
fn list_triangles_test() {
    let mut graph = Graph::complete(5);
    graph.remove_edge(0, 1);
    graph.remove_edge(3, 4);

    let mut triangles = list_triangles(&graph);
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
fn cacti_approximation_k6_test() {
    let graph = Graph::complete(6);
    let cacti = cacti_approximation(&graph);

    assert_eq!(cacti.num_of_vertices(), 6);
    assert_eq!(cacti.num_of_edges(), 15);
}
