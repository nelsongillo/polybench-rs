#![cfg_attr(not(feature = "native"), no_main)]
#![cfg_attr(not(feature = "native"), no_std)]
extern crate alloc;
use talc::*;

use polybench_rs::datamining::correlation::bench;

const SIZE: usize = 64 * 1024 * 1024;
static mut ARENA: [u8; SIZE] = [0; SIZE];
#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
    Talc::new(unsafe { ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut())) })
        .lock();

const M: usize = 600;
const N: usize = 700;

use bmvm_guest::{expose, host};

#[cfg(feature = "links1")]
seq_macro::seq!(N in 0..1 {
    #[expose]
    pub extern "C" fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[cfg(feature = "links8")]
seq_macro::seq!(N in 0..8 {
    #[expose]
    pub fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[cfg(feature = "links16")]
seq_macro::seq!(N in 0..16 {
    #[expose]
    pub fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[cfg(feature = "links32")]
seq_macro::seq!(N in 0..32 {
    #[expose]
    pub fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[cfg(feature = "links64")]
seq_macro::seq!(N in 0..64 {
    #[expose]
    pub fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[cfg(feature = "links128")]
seq_macro::seq!(N in 0..128 {
    #[expose]
    pub fn up~N() -> i32 {
        bench::<M, N>();
        N
    }
});

#[host]
unsafe extern "C" {
    #[cfg(feature = "links1")]
    seq_macro::seq!(N in 0..1 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links8")]
    seq_macro::seq!(N in 0..8 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links16")]
    seq_macro::seq!(N in 0..16 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links32")]
    seq_macro::seq!(N in 0..32 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links64")]
    seq_macro::seq!(N in 0..64 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links128")]
    seq_macro::seq!(N in 0..128 {
        fn hyper~N() -> i32;
    });
}
