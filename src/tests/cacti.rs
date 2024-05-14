use crate::cacti::*;
use crate::graphs::Graph;

fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    if k == 0 {
        return 1;
    }

    (n * binomial(n - 1, k - 1)) / k
}

//#[test]
fn _cacti_approximation_complete_test() {
    for n in 0..10 {
        println!("cacti_approximation_complete_test n = {}", n);

        let graph = Graph::complete(n);
        let cacti = cacti_approximation(&graph);

        assert_eq!(cacti.num_of_vertices(), n);
        assert_eq!(cacti.num_of_edges(), 3 * (n / 3));
    }
}
