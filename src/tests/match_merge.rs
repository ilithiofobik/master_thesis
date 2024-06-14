use crate::graphs::Graph;
use crate::match_merge::*;
use crate::mps_alg::MpsAlgorithm;

#[test]
fn calinescu_basic_mps_complete_test() {
    for n in 3..10 {
        println!("cacti_approximation_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let alg = CalinescuMps {};
        let mps = alg.maximum_planar_subgraph(&graph);

        let mut available_components = n;
        let mut num_of_edges = 0;

        while available_components >= 3 {
            available_components -= 2;
            num_of_edges += 3;
        }

        if available_components > 0 {
            num_of_edges += available_components - 1;
        }

        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), num_of_edges);
    }
}

#[test]
fn schmid_d4_mps_complete_test() {
    for (n, m) in [(3, 3), (4, 5), (5, 6), (6, 8), (7, 10), (8, 11), (9, 13)] {
        let graph = Graph::complete(n);
        let alg = SchmidMps {};
        let mps = alg.maximum_planar_subgraph(&graph);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), m);
    }
}

#[test]
fn my_mps_complete_test() {
    for (n, m) in [(3, 3), (4, 5), (5, 7), (6, 8), (7, 10), (8, 12), (9, 14)] {
        let graph = Graph::complete(n);
        let alg = MyMps {};
        let mps = alg.maximum_planar_subgraph(&graph);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), m);
    }
}

#[test]
fn poranen_mps_complete_test() {
    for (n, m) in [(3, 3), (4, 5), (5, 7), (6, 9), (7, 11), (8, 13), (9, 15)] {
        let graph = Graph::complete(n);
        let alg = PoranenMps {};
        let mps = alg.maximum_planar_subgraph(&graph);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), m);
    }
}
