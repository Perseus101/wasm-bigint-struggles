mod sys {
    extern {
        pub fn native_add(a: u64, b: u64) -> u64;
    }
}

#[no_mangle]
pub fn add(a: u64, b: u64) -> u64 {
    unsafe { sys::native_add(a, b) }
}
