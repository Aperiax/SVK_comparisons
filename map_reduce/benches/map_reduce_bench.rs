use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::f64;

fn sequential_map_reduce<'a, T>(arr: T) -> f64
where
    T: IntoIterator<Item = &'a f64>,
{
    arr.into_iter()
        .map(|x| x * x)
        // .inspect(|x| println!("{x}"))
        .sum()
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(10) // <--- reduce to 10 samples
}

fn sequential_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_reduce");
    for &size in &[10000, 100000, 1000000, 10000000] {
        let inp: Vec<f64> = (0..size).into_iter().map(|_| f64()).collect();
        group.bench_with_input(BenchmarkId::from_parameter(&size), &inp, |b, vec| {
            b.iter(|| sequential_map_reduce(vec))
        });
    }
    group.finish();
}

criterion_group! {name = benches; config = criterion_config(); targets=sequential_bench}
criterion_main!(benches);
