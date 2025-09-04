#![allow(internal_features)]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), feature(core_intrinsics))]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), no_std)]
#![cfg_attr(any(feature = "bmvm", feature = "wasm"), no_main)]

extern crate alloc;
extern crate core;

pub mod datamining;
pub mod linear_algebra;
pub mod medley;
pub mod stencils;

pub mod config;
pub mod ndarray;
pub mod util;
