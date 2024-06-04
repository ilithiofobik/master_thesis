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
