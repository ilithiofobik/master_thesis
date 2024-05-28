use crate::graphs::Graph;
use crate::poranen::*;

#[test]
fn calinescu_basic_mps_complete_test() {
    for n in 3..10 {
        println!("cacti_approximation_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let mps = calinescu_basic_mps(&graph);

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
        let mps = schmid_d4_mps(&graph);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), m);
    }
}
