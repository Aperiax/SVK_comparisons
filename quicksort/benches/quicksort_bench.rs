use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::usize;
use std::{
    array,
    fs::File,
    io::{BufReader, Read},
};

fn read_array(size: &usize) -> std::io::Result<Vec<usize>> {
    let f = File::open(format!("/home/aperiax/School/SVK/arr_{}", size))?;
    let mut reader = BufReader::new(f);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let arr: &[usize] = bytemuck::cast_slice(&buf);
    Ok(arr.to_vec())
}

fn partition(arr: &mut [usize], low: usize, high: usize) -> usize {
    let pivot: usize = arr[high];
    let mut i: usize = low;

    for j in low..high {
        let mask = (arr[j] <= pivot) as usize;
        arr.swap(i * mask + j * (1 - mask), j);
        i += mask;
    }
    arr.swap(i, high);
    i
}

fn quicksort(arr: &mut [usize], mut low: usize, mut high: usize) {
    while low < high {
        let p = partition(arr, low, high);
        if p - low < high - p {
            if p > 0 {
                quicksort(arr, low, p - 1);
            }
            low = p + 1;
        } else {
            quicksort(arr, p + 1, high);
            if p == 0 {
                break;
            }
            high = p - 1;
        }
    }
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(10) // <--- reduce to 10 samples
}

pub fn quicksort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("QS-var-size");

    for &size in &[100000, 1000000, 10000000, 50000000, 100000000] {
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &size| {
            b.iter_batched(
                || read_array(&size).ok().unwrap(),
                |mut arr| quicksort(&mut arr, 0, size - 1),
                criterion::BatchSize::SmallInput,
            );
        });
    }
}

criterion_group! {name=benches; config=criterion_config();targets= quicksort_bench}
criterion_main!(benches);
