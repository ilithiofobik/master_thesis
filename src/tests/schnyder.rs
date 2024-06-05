use crate::graphs::Graph;
use crate::rand_graphs::bliztstein_generation;
use crate::schnyder::schnyder_mps;

#[test]
fn schnyder_mps_complete_test() {
    for n in 3..8 {
        let graph = Graph::complete(n);
        let mps = schnyder_mps(&graph);
        println!("Testing Schnyder MPS for complete n = {}", n);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), 3 * n - 6);
    }
}

#[test]
fn schnyder_mps_3_regular_test() {
    for n in [4, 8] {
        let degs = vec![3; n];
        let graph = bliztstein_generation(&degs).unwrap();
        let mps = schnyder_mps(&graph);
        println!("Testing Schnyder MPS for regular n = {}", n);
        assert_eq!(mps.num_of_vertices(), n);
        assert!(mps.num_of_edges() <= 3 * n - 6);
    }
}
