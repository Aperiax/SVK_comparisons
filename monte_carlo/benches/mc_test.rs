use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::f32;
use std::f32::{self, consts::PI};

#[inline(always)]
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

pub fn mc_maxiter(c: &mut Criterion) {
    let mut group = c.benchmark_group("mc_maxiter_tests");

    for &size in &[1000000, 2000000, 3000000, 10000000] {
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &size| {
            b.iter(|| simulate(size as usize));
        });
    }
}

criterion_group!(benches, mc_maxiter);
criterion_main!(benches);
