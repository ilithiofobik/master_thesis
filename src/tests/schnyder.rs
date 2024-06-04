use crate::graphs::Graph;
use crate::schnyder::schnyder_mps;
use good_lp::*;

#[test]
fn goodlptest() {
    variables! {
        vars:
          0 <= a <= 1;
          0 <= b <= 1;
    } // variables can also be added dynamically

    let solution = vars
        .maximise(a + b)
        .using(highs) // multiple solvers available
        .with(constraint!(a <= b))
        .solve()
        .unwrap();

    assert_eq!(solution.value(a), 1.0);
    assert_eq!(solution.value(b), 1.0);
    assert_eq!(solution.eval(a + b), 2.0);
}

#[test]
fn schnyder_mps_test() {
    let mut graph = Graph::complete(5);

    let mps = schnyder_mps(&graph);
    assert_eq!(mps.num_of_vertices(), 5);
    assert_eq!(mps.num_of_edges(), 10);
}
