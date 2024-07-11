use crate::facial_walks::*;
use crate::graphs::Graph;
use crate::match_merge::*;
use crate::mps_alg::*;
use crate::rand_graphs::*;
use crate::schnyder::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

// #[test]
fn generate_3regular() {
    for n in (1..=100).map(|n| 100usize * n) {
        let d_seq = vec![3; n];
        for k in 0..10 {
            let graph = bliztstein_generation(&d_seq).unwrap();
            let name = format!("3regular_n{}_test_{}.json", n, k);
            let write = graph.write_to_json(&name);
            assert!(write.is_ok());
        }
    }
}

//#[test]
fn generate_pareto() {
    for n in (1..=100).map(|n| 100usize * n) {
        for k in 0..10 {
            let graph = random_pareto_graph(n, 2.0).unwrap();
            let name = format!("pareto_n{}_test_{}.json", n, k);
            let write = graph.write_to_json(&name);
            assert!(write.is_ok());
        }
    }
}

fn test_named_approx_algorithms(name: &str) {
    let mut output_file = File::create(format!("results/{}_output.txt", name)).unwrap();
    let algorithms: Vec<Box<dyn MpsAlgorithm>> = vec![
        Box::new(CalinescuMps {}),
        Box::new(SchmidMps {}),
        Box::new(MyMps {}),
        Box::new(PoranenMps {}),
    ];

    for n in (100..=10000).step_by(100) {
        for k in 0..=9 {
            let filename = format!("{}_n{}_test_{}.json", name, n, k);
            let graph = Graph::read_from_json(&filename);

            for alg in algorithms.iter() {
                let start = Instant::now();
                let result = alg.maximum_planar_subgraph(&graph);
                let duration = start.elapsed();
                assert!(result.is_planar());
                writeln!(
                    output_file,
                    "{},{},{},{},{},{}",
                    filename,
                    n,
                    k,
                    duration.as_nanos(),
                    result.num_of_edges(),
                    alg.name()
                )
                .unwrap();
            }
        }
    }
}

// #[test]
fn test_named_approx_complete() {
    let mut output_file = File::create(format!("results/complete_output.txt")).unwrap();
    let algorithms: Vec<Box<dyn MpsAlgorithm>> = vec![
        Box::new(CalinescuMps {}),
        Box::new(SchmidMps {}),
        Box::new(MyMps {}),
        Box::new(PoranenMps {}),
    ];

    for n in (10..=100).step_by(5) {
        let graph = Graph::complete(n);
        for k in 0..5 {
            for alg in algorithms.iter() {
                let start = Instant::now();
                let result = alg.maximum_planar_subgraph(&graph);
                let duration = start.elapsed();
                assert!(result.is_planar());
                writeln!(
                    output_file,
                    "{},{},{},{},{},{}",
                    format!("complete_n{}_test_{}", n, k),
                    n,
                    k,
                    duration.as_nanos(),
                    result.num_of_edges(),
                    alg.name()
                )
                .unwrap();
            }
        }
    }
}

// #[test]
fn test_named_exact_complete() {
    let mut output_file = File::create(format!("results/exact_complete_output.txt")).unwrap();
    let algorithms: Vec<Box<dyn MpsAlgorithm>> =
        vec![Box::new(SchnyderMps {}), Box::new(FacialWalksMps {})];

    for n in 3..=14 {
        let graph = Graph::complete(n);
        for k in 0..3 {
            for alg in algorithms.iter() {
                let start = Instant::now();
                let result = alg.maximum_planar_subgraph(&graph);
                let duration = start.elapsed();
                assert!(result.is_planar());
                writeln!(
                    output_file,
                    "{},{},{},{},{},{}",
                    format!("complete_n{}_test_{}", n, k),
                    n,
                    k,
                    duration.as_nanos(),
                    result.num_of_edges(),
                    alg.name()
                )
                .unwrap();
            }
        }
    }
}

// #[test]
fn test_named_mixed_regular() {
    let mut output_file = File::create(format!("results/mixed_regular_output.txt")).unwrap();
    let algorithms: Vec<Box<dyn MpsAlgorithm>> = vec![
        Box::new(CalinescuMps {}),
        Box::new(SchmidMps {}),
        Box::new(MyMps {}),
        Box::new(PoranenMps {}),
        Box::new(SchnyderMps {}),
        Box::new(FacialWalksMps {}),
    ];

    for n in (4..=20).step_by(2) {
        let d_in = vec![3; n];
        for k in 0..3 {
            let graph = bliztstein_generation(&d_in).unwrap();
            for alg in algorithms.iter() {
                let start = Instant::now();
                let result = alg.maximum_planar_subgraph(&graph);
                let duration = start.elapsed();
                assert!(result.is_planar());
                writeln!(
                    output_file,
                    "{},{},{},{},{},{}",
                    format!("regular_n{}_test_{}", n, k),
                    n,
                    k,
                    duration.as_nanos(),
                    result.num_of_edges(),
                    alg.name()
                )
                .unwrap();
            }
        }
    }
}

// #[test]
fn test_approx_algorithms() {
    test_named_approx_algorithms("3regular");
    test_named_approx_algorithms("pareto");
}
