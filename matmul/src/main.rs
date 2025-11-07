use std::time::{Duration, Instant};

use ndarray::Array2;
use ndarray::linalg::general_mat_mul;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator,
};
use std::io::BufRead;
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
};

// matrix is basically just a vector of vectors!
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

    pub fn output_matrix_for_python() -> std::io::Result<()> {
        for &ident in &[1, 2] {
            for &size in &[10, 100, 1000, 3000, 5000] {
                let data_1: Vec<f32> = (0..size * size).map(|_| fastrand::f32()).collect();
                let file = File::create(format!(
                    "/home/aperiax/School/SVK/matrix_{}_{}",
                    ident, size
                ))?;
                let mut writer = BufWriter::new(file);

                for a in data_1 {
                    writeln!(writer, "{}", a)?;
                }
                writer.flush()?;
            }
        }
        Ok(())
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

fn measure_raw<T>(mut f: impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    let _ = f();

    Instant::now() - start
}

fn main() {
    Matrix::output_matrix_for_python().expect("Something went wrong");
    //     let num_runs: usize = 10;
    //     let mut res_rayon: Vec<f64> = Vec::with_capacity(num_runs);
    //     let mut res_naive: Vec<f64> = Vec::with_capacity(num_runs);
    //
    //     // measure one-shot raws
    //     for _ in 0..num_runs {
    //         let mut temp_naive: Vec<Duration> = Vec::with_capacity(num_runs);
    //         let mut temp_rayon: Vec<Duration> = Vec::with_capacity(num_runs);
    //
    //         for &size in &[10, 100, 1000, 3000, 5000] {
    //             let M1 = Matrix::read_test_matrix(1, size).expect("matrix 1 read fail");
    //             let M2 = Matrix::read_test_matrix(2, size).expect("matrix 2 read fail");
    //
    //             temp_rayon.push(measure_raw(|| M1.matmul_naive(&M2)));
    //             temp_naive.push(measure_raw(|| M1.matmul_naive(&M2)));
    //         }
    //
    //         res_rayon.push(
    //             temp_rayon.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_rayon.len() as f64,
    //         );
    //         res_naive.push(
    //             temp_naive.iter().map(|i| i.as_secs_f64()).sum::<f64>() / temp_naive.len() as f64,
    //         );
    //     }
    //
    //     println!("MATMUL");
    //     println!("Tested sizes: 10, 100, 1000, 3000, 5000");
    //     println!("Averages sequential, 10 runs: {:?}", res_naive);
    //     println!("Averages rayon, 10 runs: {:?}", res_rayon);
    //     println!("Done");
}
