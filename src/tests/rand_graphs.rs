use crate::rand_graphs::bliztstein_generation;

fn blizstein_test(d: &[usize]) {
    let graph = bliztstein_generation(&d).unwrap();
    assert_eq!(graph.num_of_edges(), d.iter().sum::<usize>() / 2);

    for u in 0..d.len() {
        match graph.neighbours(u) {
            Some(neighbours) => {
                assert_eq!(neighbours.len(), d[u]);
            }
            None => assert!(false, "Vertex {} not found", u),
        }
    }
}

#[test]
fn bliztstein_generation_regular_test() {
    let d = vec![3; 10];
    blizstein_test(&d);
}

#[test]
fn bliztstein_generation_test() {
    let d = vec![3, 2, 2, 2, 1];
    blizstein_test(&d);
}
