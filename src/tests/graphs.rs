use crate::graphs::Graph;

#[test]
fn adding_edge() {
    let mut graph = Graph::empty(2);
    assert!(graph.add_edge(0, 1));
    assert!(!graph.add_edge(2, 3));
    assert_eq!(graph.num_of_edges(), 1);
}
