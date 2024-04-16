use crate::rand_graphs::random_with_degree_seq;

#[test]
fn adding_edge() {
    let d = vec![3; 100];
    let (graph, try_num) = random_with_degree_seq(&d).unwrap();
    assert_eq!(graph.num_of_edges(), d.iter().sum::<usize>() / 2);
    assert_eq!(try_num, 1);
}
