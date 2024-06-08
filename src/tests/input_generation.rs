use crate::cacti::*;
use crate::graphs::Graph;
use crate::mps_alg::*;
use crate::poranen::*;
use crate::rand_graphs::*;

//#[test]
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

// #[test]
fn generate_regular() {
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

#[test]
fn test_regular_algorithms() {
    let mut output_file = File::create("results/3regular_output.txt").unwrap();
    let algorithms: Vec<Box<dyn MpsAlgorithm>> =
        vec![CalinescuMps {}, SchmidMps {}, MyMps {}, PoranenMps {}];

    for n in (100..=10000).step_by(100) {
        for k in 1..=9 {
            let filename = format!("3regular_n{}_test_{}.json", n, k);
            let graph = open_graph(&filename)?;

            for alg in algorithms.iter() {
                let start = Instant::now();
                let result = alg.maximum_planar_subgraph(&graph);
                let duration = start.elapsed();
                writeln!(
                    output_file,
                    "{},{},{},{:?},{},{}",
                    filename,
                    n,
                    k,
                    duration,
                    result,
                    alg.name()
                )?;
            }
        }
    }
}
