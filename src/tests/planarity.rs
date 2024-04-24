use crate::graphs::Graph;
use crate::planarity::is_planar;

#[test]
fn k4_test() {
    let graph = Graph::complete(4);
    assert!(is_planar(&graph));
}

#[test]
fn tarjan_test() {
    let mut graph = Graph::empty(9);
    graph.add_edge(0, 1);
    graph.add_edge(0, 4);
    graph.add_edge(0, 5);
    graph.add_edge(0, 6);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(4, 5);
    graph.add_edge(6, 8);
    graph.add_edge(6, 7);
    graph.add_edge(7, 8);
    assert!(is_planar(&graph));
}
