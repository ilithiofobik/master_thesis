use crate::graphs::Graph;

#[test]
fn adding_edge() {
    let mut graph = Graph::empty(2);
    assert!(graph.add_edge(0, 1));
    assert!(!graph.add_edge(2, 3));
    assert_eq!(graph.num_of_edges(), 1);
}

#[test]
fn create_bipartite_complete() {
    let graph = Graph::bipartite_complete(3, 3);
    assert_eq!(graph.num_of_vertices(), 6);
    assert_eq!(graph.num_of_edges(), 9);
    assert!(!graph.has_edge(0, 0));
    assert!(!graph.has_edge(0, 1));
    assert!(!graph.has_edge(0, 2));
    assert!(graph.has_edge(0, 3));
    assert!(graph.has_edge(0, 4));
    assert!(graph.has_edge(0, 5));
}
