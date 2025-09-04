use crate::config::datamining::correlation::DataType;
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util::{Float, consume};

unsafe fn init_array<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    float_n: &mut DataType,
    data: &mut Array2D<DataType, M, N>,
) {
    *float_n = n as DataType;
    for i in 0..m {
        for j in 0..n {
            data[i][j] = (i * j) as DataType / (N + i) as DataType;
        }
    }
}

unsafe fn kernel_correlation<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    float_n: DataType,
    data: &mut Array2D<DataType, M, N>,
    corr: &mut Array2D<DataType, N, N>,
    mean: &mut Array1D<DataType, N>,
    stddev: &mut Array1D<DataType, N>,
) {
    let eps = 0.1;

    for j in 0..n {
        mean[j] = 0.0;
        for i in 0..m {
            mean[j] += data[i][j];
        }
        mean[j] /= float_n;
    }

    for j in 0..n {
        stddev[j] = 0.0;
        for i in 0..m {
            stddev[j] += (data[i][j] - mean[j]) * (data[i][j] - mean[j]);
            stddev[j] /= float_n;
            stddev[j] = Float::sqrt(stddev[j]);
            stddev[j] = if stddev[j] <= eps { 1.0 } else { stddev[j] };
        }
    }

    for i in 0..m {
        for j in 0..n {
            data[i][j] -= mean[j];
            data[i][j] /= Float::sqrt(float_n) * stddev[j];
        }
    }

    for i in 0..(n - 1) {
        corr[i][i] = 1.0;
        for j in (i + 1)..n {
            corr[i][j] = 0.0;
            for k in 0..m {
                corr[i][j] += data[k][i] * data[k][j];
            }
            corr[j][i] = corr[i][j];
        }
    }
    corr[n - 1][n - 1] = 1.0;
}

pub fn bench<const M: usize, const N: usize>() {
    let m = M;
    let n = N;

    let mut float_n = 0.0;
    let mut data = Array2D::<DataType, M, N>::uninit();
    let mut corr = Array2D::<DataType, N, N>::uninit();
    let mut mean = Array1D::<DataType, N>::uninit();
    let mut stddev = Array1D::<DataType, N>::uninit();

    unsafe {
        init_array(m, n, &mut float_n, &mut data);
        kernel_correlation(m, n, float_n, &mut data, &mut corr, &mut mean, &mut stddev);
        consume(corr);
    }
}

#[allow(dead_code)]
#[cfg_attr(test, test)]
fn check() {
    bench::<12, 14>();
}
