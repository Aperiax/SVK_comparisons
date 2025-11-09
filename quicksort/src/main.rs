use bytemuck::cast_slice;
use fastrand::usize;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    time::{Duration, Instant},
};

pub fn output_arrays() -> std::io::Result<()> {
    for &size in &[100000, 1000000, 10000000, 50000000, 100000000] {
        let mut arr: Vec<usize> = (0..size).into_iter().map(|_| usize(0..size)).collect();

        let file = File::create(format!("/home/aperiax/School/SVK/arr_{}", size))?;
        let mut writer = BufWriter::new(file);

        writer.write_all(bytemuck::cast_slice(&arr))?;
        writer.flush()?;
    }
    Ok(())
}

fn read_array(size: &usize) -> std::io::Result<Vec<usize>> {
    let f = File::open(format!("/home/aperiax/School/SVK/arr_{}", size))?;
    let mut reader = BufReader::new(f);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let arr: &[usize] = cast_slice(&buf);
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

fn check_correct(arr: &[usize]) {
    for ind in 0..arr.len() - 1 {
        assert!(arr[ind] <= arr[ind + 1], "Wrong order!")
    }
}

fn measure_raw<T>(mut f: impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    let r = f();
    Instant::now() - start
}

fn main() {
    // output_arrays().ok();
    let num_runs: usize = 10;
    let mut res_vec: Vec<f64> = Vec::with_capacity(5);
    for &size in &[100000, 1000000, 10000000, 50000000, 100000000] {
        // println!("{size}");
        let n = size - 1;
        let mut temp: Vec<Duration> = Vec::with_capacity(10);
        for run in 0..num_runs {
            // println!("{run}");
            let mut arr: Vec<usize> = read_array(&size).ok().unwrap();
            println!("{:?}", &arr[0..20]);
            let res = measure_raw(|| quicksort(&mut arr, 0, n));
            check_correct(&arr);
            temp.push(res)
        }
        res_vec.push(temp.iter().map(|dur| dur.as_secs_f64()).sum::<f64>() / 10_f64);
    }

    println!("QUICKSORT");
    println!("tested sizes: 1e5, 1e6, 1e5, 5e7, 1e8");
    println!("Results quickshot: {:?}", res_vec);
    println!("QUICKSORT");
}
