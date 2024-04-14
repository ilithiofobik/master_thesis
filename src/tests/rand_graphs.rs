use crate::rand_graphs::random_with_degree_seq;

#[test]
fn adding_edge() {
    let d = vec![5, 5, 5, 5, 5, 5];
    let graph = random_with_degree_seq(&d).unwrap();
    assert_eq!(graph.num_of_edges(), 15);
}
