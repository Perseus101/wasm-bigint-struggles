use wasm_bindgen::prelude::*;

mod sys {
    extern {
        pub fn native_add(a: u64, b: u64) -> u64;
    }
}

#[wasm_bindgen]
pub fn add(a: u64, b: u64) -> u64 {
    unsafe { sys::native_add(a, b) }
}
