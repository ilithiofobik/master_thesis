use crate::graphs::Graph;

#[test]
fn generate_complete() {
    let small = (1..100).map(|n| 10 * n).collect::<Vec<usize>>();
    let large = (1..=100).map(|n| 100 * n).collect::<Vec<usize>>();
    let both = small.iter().chain(large.iter());

    for n in both {
        let graph = Graph::complete(*n);
        let name = format!("k{}_test.json", n);
        let write = graph.write_to_json(&name);
        assert!(write.is_ok());
    }
}
