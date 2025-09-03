#![cfg_attr(not(feature = "native"), no_main)]
#![cfg_attr(not(feature = "native"), no_std)]
extern crate alloc;

use polybench_rs::stencils::heat_3d::bench;

use talc::*;

const SIZE: usize = 64 * 1024 * 1024;
static mut ARENA: [u8; SIZE] = [0; SIZE];
#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
    Talc::new(unsafe { ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut())) })
        .lock();

#[cfg_attr(feature = "bmvm", bmvm_guest::expose)]
#[cfg_attr(feature = "wasm", inline)]
#[cfg(not(feature = "native"))]
pub extern "C" fn run() {
    bench::<120, 500>();
}

#[cfg(feature = "native")]
fn main() {
    let now = std::time::Instant::now();
    bench::<120, 500>();
    let elapsed = now.elapsed();
    print!("{}", elapsed.as_nanos());
}
