use crate::graphs::Graph;
use crate::planarity::is_planar;

#[test]
fn k4_test() {
    let graph = Graph::complete(4);
    assert!(is_planar(&graph));
}
