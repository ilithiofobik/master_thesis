use crate::rand_graphs::random_with_degree_seq;

#[test]
fn adding_edge() {
    let d = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    let graph = random_with_degree_seq(&d).unwrap();
    assert_eq!(graph.num_of_edges(), 15);
}
