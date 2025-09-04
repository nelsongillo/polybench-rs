#![cfg_attr(not(feature = "native"), no_main)]
#![cfg_attr(not(feature = "native"), no_std)]
extern crate alloc;
extern crate core;

use talc::*;

const SIZE: usize = 64 * 1024 * 1024;
static mut ARENA: [u8; SIZE] = [0; SIZE];
#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
    Talc::new(unsafe { ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut())) })
        .lock();

use polybench_rs::linear_algebra::kernels::_3mm::bench;

const NI: usize = 400;
const NJ: usize = 450;
const NK: usize = 500;
const NL: usize = 550;
const NM: usize = 600;

#[cfg_attr(feature = "bmvm", bmvm_guest::expose)]
#[unsafe(no_mangle)]
pub extern "C" fn run() {
    bench::<NI, NJ, NK, NL, NM>();
}

#[cfg(feature = "native")]
fn main() {
    let now = std::time::Instant::now();
    bench::<NI, NJ, NK, NL, NM>();
    let elapsed = now.elapsed();
    print!("{}", elapsed.as_nanos());
}
