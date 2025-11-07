use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::f64;
use rayon::prelude::*;

fn rayon_map_reduce<'a, T>(arr: T) -> f64
where
    T: IntoParallelIterator<Item = &'a f64>,
{
    arr.into_par_iter()
        .map(|x| x * x)
        // .inspect(|x| println!("{x}"))
        .sum()
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(10) // <--- reduce to 10 samples
}

fn rayon_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("rayon_reduce");
    for &size in &[10000, 100000, 1000000, 10000000] {
        let inp: Vec<f64> = (0..size).into_iter().map(|_| f64()).collect();
        group.bench_with_input(BenchmarkId::from_parameter(&size), &inp, |b, vec| {
            b.iter(|| rayon_map_reduce(vec))
        });
    }
    group.finish();
}

criterion_group! {name = benches; config = criterion_config(); targets=rayon_bench}
criterion_main!(benches);
