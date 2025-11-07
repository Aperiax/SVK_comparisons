use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::f32;
use ndarray::Array2;
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};

use std::io::BufRead;
use std::{
    fs::File,
    io::{self, BufReader},
};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl Matrix {
    /// Prepare an empty matrix
    pub fn new_empty(nrows: usize, ncols: usize) -> Self {
        let width: usize = nrows * ncols;
        let data: Vec<f32> = Vec::with_capacity(width);

        Matrix {
            rows: nrows,
            cols: ncols,
            data: data,
        }
    }

    /// processes the collection into a square matrix for demonstration purposes
    pub fn try_from_collection<T>(ncols: usize, nrows: usize, collection: T) -> Result<Self, String>
    where
        T: IntoIterator<Item = f32>,
    {
        let data: Vec<f32> = collection.into_iter().collect();
        if nrows * ncols != data.len() {
            return Err(
                "Wrong combination of columns/rows to convert collection into matrix".to_string(),
            );
        }

        Ok(Matrix {
            rows: nrows,
            cols: ncols,
            data,
        })
    }

    pub fn try_from_array<const N: usize>(
        ncols: usize,
        nrows: usize,
        arr: [f32; N],
    ) -> Result<Self, String> {
        if nrows * ncols != N {
            return Err(
                "Wrong combination of columns/rows to convert collection into matrix".to_string(),
            );
        };

        let data = arr.into_iter().collect::<Vec<f32>>();

        Ok(Matrix {
            rows: nrows,
            cols: ncols,
            data,
        })
    }

    pub fn matmul_naive(&self, other: &Matrix) -> Matrix {
        // c_ij = sum_{k=1}^n a_ik * b_ik
        // access indices in row-major order

        assert!(self.cols == other.rows, "Dimension mismatch for matmul");

        let mut tmp_: Vec<f32> = Vec::with_capacity(self.rows * self.cols);
        // the second matrix has to also be row-major, just like the first one
        // well it doesn't really matter for it to be row-major really.

        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut acc = 0_f32;
                for i in 0..self.cols {
                    let a = self.data[r * self.cols + i];
                    let b = other.data[i * other.cols + c];

                    acc += a * b;
                }
                tmp_.push(acc);
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: tmp_,
        }
    }

    pub fn read_test_matrix(ident: usize, size: usize) -> io::Result<Matrix> {
        let path = format!("/home/aperiax/School/SVK/matrix_{}_{}", ident, size);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut data: Vec<f32> = Vec::with_capacity(ident * ident);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let a: f32 = parts.next().unwrap().parse().unwrap();
            data.push(a);
        }
        let m = Matrix::try_from_collection(ident, ident, data).expect("failed to create matrix");
        Ok(m)
    }

    pub fn matmul_rayon(&self, other: &Matrix) -> Matrix {
        // I'm not outgunning BLAS/LAPAC, but i can get kind of close-ish without much hassle
        // for medium-sized matrices.
        let mut tmp_: Vec<f32> = Vec::with_capacity(self.cols * other.rows);

        tmp_.par_extend((0..self.rows).into_par_iter().flat_map(|r| {
            (0..other.cols).into_par_iter().map(move |c| {
                let mut acc: f32 = 0.;
                for i in 0..self.cols {
                    let a = self.data[r * self.cols + i];
                    let b = other.data[i * other.cols + c];

                    acc += a * b
                }
                acc
            })
        }));

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: tmp_,
        }
    }
}

pub fn matmul_NdArray(M1: Matrix, M2: Matrix) -> Array2<f32> {
    // cast the two matrices into the ndarray counterpart first
    let first = Array2::from_shape_vec((M1.rows, M1.cols), M1.data)
        .ok()
        .unwrap();
    let second = Array2::from_shape_vec((M2.rows, M2.cols), M2.data)
        .ok()
        .unwrap();
    let mut c = Array2::<f32>::zeros((M1.rows, M2.cols));

    let res = first.dot(&second);
    res
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(10) // <--- reduce to 10 samples
}

fn naive_seq_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("naive-benches");

    for &size in &[10, 100, 1000, 3000] {
        let M1: Matrix = Matrix::read_test_matrix(1, size).expect("matrix 1 read failed");
        let M2: Matrix = Matrix::read_test_matrix(2, size).expect("matrix 2 read failed");

        let inp = (M1, M2);
        group.bench_with_input(BenchmarkId::from_parameter(size), &inp, |b, i| {
            b.iter(|| i.0.matmul_naive(&i.1))
        });
    }
}

criterion_group! {name = benches; config = criterion_config(); targets=naive_seq_bench}
criterion_main!(benches);
