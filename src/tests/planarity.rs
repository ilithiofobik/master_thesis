use crate::facial_walks::*;
use crate::graphs::Graph;
use crate::match_merge::*;
use crate::mps_alg::*;
use crate::rand_graphs::*;
use crate::schnyder::*;

#[test]
fn k4_test() {
    let graph = Graph::complete(4);
    assert!(graph.is_planar());
}

#[test]
fn k5_test() {
    let graph = Graph::complete(5);
    assert!(!graph.is_planar());
}

#[test]
fn k6_test() {
    let graph = Graph::complete(6);
    assert!(!graph.is_planar());
}

#[test]
fn k5_minus_edge_test() {
    let mut graph = Graph::complete(5);
    graph.remove_edge(0, 1);
    assert!(graph.is_planar());
}

#[test]
fn k23_test() {
    let graph = Graph::bipartite_complete(2, 3);
    graph.print_edges();
    assert!(graph.is_planar());
}

#[test]
fn k33_test() {
    let graph = Graph::bipartite_complete(3, 3);
    graph.print_edges();
    assert!(!graph.is_planar());
}

#[test]
fn k34_test() {
    let graph = Graph::bipartite_complete(3, 4);
    graph.print_edges();
    assert!(!graph.is_planar());
}

#[test]
fn k33_minus_edge_test() {
    let mut graph = Graph::bipartite_complete(3, 3);
    assert!(graph.remove_edge(0, 3));
    assert!(graph.is_planar());
}

#[test]
fn random_graph_deg2_test() {
    let graph = random_regular_graph(1000, 2);
    assert!(graph.is_ok());
    assert!(graph.unwrap().is_planar());
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
    assert!(graph.is_planar());
}

#[test]
fn test_named_approx_planarity() {
    let algorithms: Vec<Box<dyn MpsAlgorithm>> = vec![
        Box::new(CalinescuMps {}),
        Box::new(SchmidMps {}),
        Box::new(MyMps {}),
        Box::new(PoranenMps {}),
    ];

    for n in 1..=10 {
        let pareto = random_pareto_graph(100 * n, 2.0).unwrap();
        let regular = random_regular_graph(100 * n, 3).unwrap();
        let complete = Graph::complete(n);

        for alg in algorithms.iter() {
            let p_result = alg.maximum_planar_subgraph(&pareto);
            let r_result = alg.maximum_planar_subgraph(&regular);
            let c_result = alg.maximum_planar_subgraph(&complete);

            assert!(p_result.is_planar());
            assert!(r_result.is_planar());
            assert!(c_result.is_planar());
        }
    }
}

#[test]
fn test_named_exact_planarity() {
    let algorithms: Vec<Box<dyn MpsAlgorithm>> =
        vec![Box::new(SchnyderMps {}), Box::new(FacialWalksMps {})];

    let regular = random_regular_graph(6, 3).unwrap();
    let complete = Graph::complete(4);

    for alg in algorithms.iter() {
        let r_result = alg.maximum_planar_subgraph(&regular);
        let c_result = alg.maximum_planar_subgraph(&complete);

        assert!(r_result.is_planar());
        assert!(c_result.is_planar());
    }
}
