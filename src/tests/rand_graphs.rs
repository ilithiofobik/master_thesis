use crate::rand_graphs::bliztstein_generation;

fn bliztstein_test(d: &[usize]) {
    let graph = bliztstein_generation(d).unwrap();
    assert_eq!(graph.num_of_edges(), d.iter().sum::<usize>() / 2);

    for (u, &du) in d.iter().enumerate() {
        match graph.neighbors(u) {
            Some(neighbors) => {
                assert_eq!(neighbors.len(), du);
            }
            None => panic!("Vertex {} not found", u),
        }
    }
}

#[test]
fn bliztstein_generation_regular_test() {
    let d = vec![10; 1000];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_test() {
    let d = vec![
        7, 8, 5, 1, 1, 2, 8, 10, 4, 2, 4, 5, 3, 6, 7, 3, 2, 7, 6, 1, 2, 9, 6, 1, 3, 4, 6, 3, 3, 3,
        2, 4, 4,
    ];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_complete_test() {
    let d = vec![99; 100];
    bliztstein_test(&d);
}

#[test]
fn bliztstein_generation_non_graphical_test() {
    let d = vec![4, 3, 2, 1, 0];
    assert!(bliztstein_generation(&d).is_err());
}
