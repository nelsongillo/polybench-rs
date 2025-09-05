#![allow(non_snake_case)]

use crate::config::stencils::seidel_2d::DataType;
use crate::ndarray::{Array2D, ArrayAlloc};
// use crate::util::consume;

unsafe fn init_array<const N: usize, const TSTEPS: usize>(
    n: usize,
    A: &mut Array2D<DataType, N, N>,
) {
    for i in 0..n {
        for j in 0..n {
            A[i][j] = (i * (j + 2) + 2) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_seidel_2d<const N: usize, const TSTEPS: usize>(
    tsteps: usize,
    n: usize,
    A: &mut Array2D<DataType, N, N>,
) {
    for _ in 0..tsteps {
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                A[i][j] = (A[i - 1][j - 1]
                    + A[i - 1][j]
                    + A[i - 1][j + 1]
                    + A[i][j - 1]
                    + A[i][j]
                    + A[i][j + 1]
                    + A[i + 1][j - 1]
                    + A[i + 1][j]
                    + A[i + 1][j + 1])
                    / 9.0;
            }
        }
    }
}

pub fn bench<const N: usize, const TSTEPS: usize>() {
    let n = N;
    let tsteps = TSTEPS;

    let mut A = Array2D::<DataType, N, N>::uninit();

    unsafe {
        init_array::<N, TSTEPS>(n, &mut A);
        kernel_seidel_2d::<N, TSTEPS>(tsteps, n, &mut A);
        // consume(A);
    }
}
#[allow(dead_code)]
#[cfg_attr(test, test)]
fn check() {
    bench::<20, 5>();
}
