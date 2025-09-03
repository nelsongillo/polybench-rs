use crate::config::linear_algebra::solvers::durbin::DataType;
use crate::ndarray::{Array1D, ArrayAlloc};
use crate::util;

unsafe fn init_array<const N: usize>(n: usize, r: &mut Array1D<DataType, N>) {
    for i in 0..n {
        r[i] = (n + 1 - i) as DataType;
    }
}

unsafe fn kernel_durbin<const N: usize>(
    n: usize,
    r: &Array1D<DataType, N>,
    y: &mut Array1D<DataType, N>,
) {
    let mut z: [DataType; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    y[0] = -r[0];
    let mut beta = 1.0;
    let mut alpha = -r[0];
    for k in 1..n {
        beta = (1.0 - alpha * alpha) * beta;
        let mut sum = 0.0;
        for i in 0..k {
            sum += r[k - i - 1] * y[i];
        }
        alpha = -(r[k] + sum) / beta;

        for i in 0..k {
            z[i] = y[i] + alpha * y[k - i - 1];
        }
        for i in 0..k {
            y[i] = z[i];
        }
        y[k] = alpha;
    }
}

pub fn bench<const N: usize>() {
    let n = N;

    let mut r = Array1D::<DataType, N>::uninit();
    let mut y = Array1D::<DataType, N>::uninit();

    unsafe {
        init_array(n, &mut r);
        kernel_durbin(n, &r, &mut y);
        util::consume(y);
    }
}
#[allow(dead_code)]
#[cfg_attr(test, test)]
fn check() {
    bench::<20>();
}
