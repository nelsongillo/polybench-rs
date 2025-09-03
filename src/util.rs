pub fn consume<T>(dummy: T) -> T {
    unsafe {
        // Taken from bencher crate:
        // https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596
        let ret = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        ret
    }
}
