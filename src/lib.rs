#![allow(internal_features)]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), feature(core_intrinsics))]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), no_std)]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), no_main)]

extern crate alloc;
extern crate core;

#[cfg(any(feature = "bmvm", feature = "wasm"))]
use core::intrinsics;

pub mod datamining;
pub mod linear_algebra;
pub mod medley;
pub mod stencils;

pub mod config;
pub mod ndarray;
pub mod util;

trait Float {
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn powf(self, n: Self) -> Self;
}

#[cfg(any(feature = "bmvm", feature = "wasm"))]
impl Float for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { intrinsics::sqrtf64(self) }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        unsafe { intrinsics::expf64(self) }
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        unsafe { intrinsics::powf64(self, n) }
    }
}

#[cfg(not(any(feature = "bmvm", feature = "wasm")))]
impl Float for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        std::primitive::f64::sqrt(self)
    }

    #[inline(always)]
    fn exp(self) -> Self {
        std::primitive::f64::exp(self)
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        std::primitive::f64::powf(self, n)
    }
}

#[cfg(any(feature = "bmvm", feature = "wasm"))]
impl Float for f32 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { intrinsics::sqrtf32(self) }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        unsafe { intrinsics::expf32(self) }
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        unsafe { intrinsics::powf32(self, n) }
    }
}

#[cfg(not(any(feature = "bmvm", feature = "wasm")))]
impl Float for f32 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        std::primitive::f32::sqrt(self)
    }

    #[inline(always)]
    fn exp(self) -> Self {
        std::primitive::f32::exp(self)
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        std::primitive::f32::powf(self, n)
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // You can call `unreachable!()` or loop forever
    loop {}
}
