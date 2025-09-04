pub fn consume<T>(dummy: T) -> T {
    unsafe {
        // Taken from bencher crate:
        // https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596
        let ret = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        ret
    }
}

pub(crate) trait Float {
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn powf(self, n: Self) -> Self;
}

#[cfg(any(feature = "bmvm", feature = "wasm"))]
impl Float for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { core::intrinsics::sqrtf64(self) }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        unsafe { core::intrinsics::expf64(self) }
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        unsafe { core::intrinsics::powf64(self, n) }
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
        unsafe { core::intrinsics::sqrtf32(self) }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        unsafe { core::intrinsics::expf32(self) }
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        unsafe { core::intrinsics::powf32(self, n) }
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
    unreachable!();
}
