use crate::graphs::Graph;
use crate::poranen::*;

#[test]
fn basic_cacti_approximation_complete_test() {
    for n in 3..10 {
        println!("cacti_approximation_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let cacti = calinescu_basic_mps(&graph);

        let mut available_components = n;
        let mut num_of_edges = 0;

        while available_components >= 3 {
            available_components -= 2;
            num_of_edges += 3;
        }

        if available_components > 0 {
            num_of_edges += available_components - 1;
        }

        assert_eq!(cacti.num_of_vertices(), n);
        assert_eq!(cacti.num_of_edges(), num_of_edges);
    }
}
