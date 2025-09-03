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

#[cfg_attr(feature = "bmvm", bmvm_guest::expose)]
#[unsafe(no_mangle)]
pub extern "C" fn run() {
    bench::<800, 900, 1000, 1100, 1200>();
}

#[cfg(feature = "native")]
fn main() {
    let now = std::time::Instant::now();
    bench::<800, 900, 1000, 1100, 1200>();
    let elapsed = now.elapsed();
    print!("{}", elapsed.as_nanos());
}
