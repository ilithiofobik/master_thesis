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
    let d = vec![10; 100];
    blizstein_test(&d);
}

#[test]
fn bliztstein_generation_test() {
    let d = vec![
        7, 8, 5, 1, 1, 2, 8, 10, 4, 2, 4, 5, 3, 6, 7, 3, 2, 7, 6, 1, 2, 9, 6, 1, 3, 4, 6, 3, 3, 3,
        2, 4, 4,
    ];
    blizstein_test(&d);
}
