use crate::graph::Graph;
use std::time::{Duration, Instant};
mod graph;

fn measure_raw<T>(mut f: impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    let _ = f();

    Instant::now() - start
}

fn main() {
    // Graph::output_graph_for_bfs_testing().expect("Failed to write into file");
    let num_runs: usize = 10;
    let mut res_graphgen: Vec<f64> = Vec::with_capacity(4);
    let mut res_bfs: Vec<f64> = Vec::with_capacity(4);

    // measure one-shot raws
    for &size in &[100, 1000, 10000, 100000] {
        let mut temp_graphgen: Vec<Duration> = Vec::with_capacity(num_runs);
        let mut temp_bfs: Vec<Duration> = Vec::with_capacity(num_runs);

        for _ in 0..num_runs {
            let mut g = Graph::new(size);
            temp_graphgen.push(measure_raw(|| g.complete_graph(0.02)));
            drop(g);

            let g = Graph::read_test_graph(size).expect("Failed to open graph");
            temp_bfs.push(measure_raw(|| g.bfs(0, fastrand::usize(0..size - 1))));
        }

        res_graphgen.push(
            temp_graphgen.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_graphgen.len() as f64,
        );

        res_bfs.push(temp_bfs.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_bfs.len() as f64);
    }

    println!("BFS+GRAPHGEN");
    println!("Tested sizes: [100, 1000, 10000, 100000]");
    println!("Averages graphgen, 10 runs: {:?}", res_graphgen);
    println!("Averages bfs, 10 runs: {:?}", res_bfs);
    println!("Done");
}
