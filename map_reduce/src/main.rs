use rayon::prelude::*;
use std::arch::x86_64::*;
use std::time::{Duration, Instant};

// lifetime specifier and generic type allows
// us to use this for any collection allowing
// iteration by reference (vecdeques, hahsmaps, vecs, arrs)
fn sequential_map_reduce<'a, T>(col: T) -> f64
where
    T: IntoIterator<Item = &'a f64>,
{
    col.into_iter()
        .map(|x| x * x)
        // .inspect(|x| println!("{x}"))
        .sum()
}

//also like 2 s,
fn rayon_map_reduce<'a, T>(arr: T) -> f64
where
    T: IntoParallelIterator<Item = &'a f64>,
{
    //TODO: try this with chunking! to see if I can crank the num_threads up
    arr.into_par_iter()
        .map(|x| x * x)
        // .inspect(|x| println!("{x}"))
        .sum()
}

#[inline(always)]
//NOTE: doesn't really do that well
fn simd_optim_sum(data: &[f64]) -> f64 {
    let total: f64 = unsafe {
        data.par_chunks(1024)
            .map(|thread_chunk| {
                let mut sum = _mm256_setzero_pd();

                for subchunk in thread_chunk.chunks_exact(4) {
                    let x = _mm256_loadu_pd(subchunk.as_ptr());
                    let prod = _mm256_mul_pd(x, x);
                    sum = _mm256_add_pd(sum, prod);
                }

                let mut tmp = [0_f64; 4];

                _mm256_store_pd(tmp.as_mut_ptr(), sum);

                let scalar_sum = tmp.iter().sum::<f64>()
                    + thread_chunk
                        .chunks_exact(4)
                        .remainder()
                        .iter()
                        .map(|f| f * f)
                        .sum::<f64>();

                scalar_sum
            })
            .sum()
    };
    return total;
}

fn measure_raw<T>(mut f: impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    let a = f();
    std::hint::black_box(a);
    Instant::now() - start
}

fn main() {
    let mut res_seq: Vec<f64> = Vec::with_capacity(4);
    let mut res_rayon: Vec<f64> = Vec::with_capacity(4);

    for &size in &[10000, 100000, 1000000, 10000000] {
        let data: Vec<f64> = (0..size).map(|_| fastrand::f64()).collect();
        let mut temp_seq: Vec<Duration> = Vec::with_capacity(10);
        let mut temp_rayon: Vec<Duration> = Vec::with_capacity(10);

        for _ in 0..10 {
            let time_seq = measure_raw(|| sequential_map_reduce(&data));
            let time_rayon = measure_raw(|| rayon_map_reduce(&data));
            temp_seq.push(time_seq);
            temp_rayon.push(time_rayon);
        }

        res_seq.push(temp_seq.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_seq.len() as f64);
        println!("{}", res_seq.len());

        res_rayon.push(
            temp_rayon.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_rayon.len() as f64,
        );
    }

    println!("MPRDC");
    println!("Measuring for 1e4, 1e5, 1e6 and 1e7 elements");
    println!("Averages sequential: {:?}", res_seq);
    println!("Averages rayon: {:?}", res_rayon);
}
