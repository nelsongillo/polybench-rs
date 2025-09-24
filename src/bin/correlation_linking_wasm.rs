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

#[unsafe(no_mangle)]
pub extern "C" fn up0() -> i32 {
    bench::<M, N>();
    unsafe { hyper0() + 1 }
}

#[cfg(feature = "links8")]
seq_macro::seq!(N in 1..8 {
    #[unsafe(no_mangle)]
     pub extern "C" fn up~N() -> i32 {
        unsafe { hyper~N() + 1 }
    }
});

#[cfg(feature = "links16")]
seq_macro::seq!(N in 1..16 {
    #[unsafe(no_mangle)]
     pub extern "C" fn up~N() -> i32 {
        unsafe { hyper~N() + 1 }
    }
});

#[cfg(feature = "links32")]
seq_macro::seq!(N in 1..32 {
    #[unsafe(no_mangle)]
     pub extern "C" fn up~N() -> i32 {
        unsafe { hyper~N() + 1 }
    }
});

#[cfg(feature = "links64")]
seq_macro::seq!(N in 1..64 {
    #[unsafe(no_mangle)]
     pub extern "C" fn up~N() -> i32 {
        unsafe { hyper~N() + 1 }
    }
});

#[cfg(feature = "links128")]
seq_macro::seq!(N in 1..128 {
    #[unsafe(no_mangle)]
     pub extern "C" fn up~N() -> i32 {
        unsafe { hyper~N() + 1 }
    }
});

unsafe extern "C" {
    fn hyper0() -> i32;

    #[cfg(feature = "links8")]
    seq_macro::seq!(N in 1..8 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links16")]
    seq_macro::seq!(N in 1..16 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links32")]
    seq_macro::seq!(N in 1..32 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links64")]
    seq_macro::seq!(N in 1..64 {
        fn hyper~N() -> i32;
    });

    #[cfg(feature = "links128")]
    seq_macro::seq!(N in 1..128 {
        fn hyper~N() -> i32;
    });
}
