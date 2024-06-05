use crate::graphs::Graph;
use crate::lr_coloring::lr_coloring_mps;
use good_lp::*;

#[test]
fn lr_coloring_mps_test() {
    for n in 3..10 {
        let graph = Graph::complete(n);
        let mps = lr_coloring_mps(&graph);
        println!("Testing LR Coloring MPS for n = {}", n);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), 3 * n - 6);
    }
}
