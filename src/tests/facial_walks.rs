use crate::facial_walks::facial_walks_mps;
use crate::graphs::Graph;

#[test]
fn facial_walks_mps_test() {
    for n in 3..8 {
        let graph = Graph::complete(n);
        let mps = facial_walks_mps(&graph);
        println!("Testing Facial Walks MPS for n = {}", n);
        assert_eq!(mps.num_of_vertices(), n);
        assert_eq!(mps.num_of_edges(), 3 * n - 6);
    }
}
