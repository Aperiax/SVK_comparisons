//! monte carlo simulation in rust, preferably sequentiall and then parallel
use std::{
    f32::{self, consts::PI},
    time::{Duration, Instant},
};

use fastrand::f32;

/// Monte carlo approximation of pi on single thread
// #[inline(always)]
fn simulate(max_iter: usize) -> (f32, i32) {
    let mut in_: f32 = 0.;
    let mut out_: f32 = 0.;

    let mut iter = 0;
    let mut res = 0_f32;

    for _ in 0..max_iter {
        let x: f32 = f32();
        let y: f32 = f32();

        // predication pattern
        let inside = (x * x + y * y < 1.) as u32;

        in_ += inside as f32;
        out_ += 1. - inside as f32;
        res = 4. * (in_ / (in_ + out_));

        // println!("{res}, diff: {diff}, in: {in_}, out: {out_}");
        iter += 1;
    }
    (res, iter)
}

fn measure_raw<T>(mut f: impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    let _ = f();
    Instant::now() - start
}

fn main() {
    let num_runs: usize = 10;
    let mut res_mc: Vec<f64> = Vec::with_capacity(num_runs);
    // let mut results: Vec<f32> = Vec::new();

    for &size in &[1000000, 2000000, 3000000, 10000000] {
        let mut temp_: Vec<Duration> = Vec::with_capacity(num_runs);
        for _ in 0..num_runs {
            temp_.push(measure_raw(|| simulate(size)));
            // results.push(simulate(size).0);
        }
        res_mc.push(temp_.iter().map(|&time| time.as_secs_f64()).sum::<f64>() / temp_.len() as f64)
    }

    println!(
        "MONTE-CARLO\nAverage results for 1e6, 2e6, 3e6 and 1e7 iters: {:?}",
        res_mc
    )
}
