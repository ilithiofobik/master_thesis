use crate::graphs::Graph;
use crate::planarity::*;
use crate::rand_graphs::bliztstein_generation;

#[test]
fn splitting_complete_graph() {
    let graph = Graph::complete(10);
    let connected_components = split_graph_into_connected(&graph);
    assert_eq!(connected_components.len(), 1);
    let component = &connected_components[0];
    assert_eq!(component.num_of_vertices(), 10);
    assert_eq!(component.num_of_edges(), 45);
}

#[test]
fn splitting_empty_graph() {
    let graph = Graph::empty(10);
    let connected_components = split_graph_into_connected(&graph);
    assert_eq!(connected_components.len(), 10);
    for component in connected_components {
        assert_eq!(component.num_of_vertices(), 1);
        assert_eq!(component.num_of_edges(), 0);
    }
}

#[test]
fn k4_test() {
    let graph = Graph::complete(4);
    assert!(is_planar(&graph));
}

#[test]
fn k5_test() {
    let graph = Graph::complete(5);
    assert!(!is_planar(&graph));
}

#[test]
fn k6_test() {
    let graph = Graph::complete(6);
    assert!(!is_planar(&graph));
}

#[test]
fn k5_minus_edge_test() {
    let mut graph = Graph::complete(5);
    graph.remove_edge(0, 1);
    assert!(is_planar(&graph));
}

#[test]
fn k23_test() {
    let graph = Graph::bipartite_complete(2, 3);
    graph.print_edges();
    assert!(is_planar(&graph));
}

// // #[test]
// fn k33_test() {
//     let graph = Graph::bipartite_complete(3, 3);
//     graph.print_edges();
//     assert!(!is_planar(&graph));
// }

// // #[test]
// fn k34_test() {
//     let graph = Graph::bipartite_complete(3, 4);
//     graph.print_edges();
//     assert!(!is_planar(&graph));
// }

#[test]
fn k33_minus_edge_test() {
    let mut graph = Graph::bipartite_complete(3, 3);
    assert!(graph.remove_edge(0, 3));
    assert!(is_planar(&graph));
}

#[test]
fn random_graph_deg2_test() {
    let graph = bliztstein_generation(&vec![2; 1000]);
    assert!(graph.is_ok());
    assert!(is_planar(&graph.unwrap()));
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
