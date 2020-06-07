use wasm_bindgen::prelude::*;
use js_sys::Object;

#[wasm_bindgen(module = "/src/imports.js")]
extern "C" {
    pub type Imports;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Imports;

    pub fn make_imports() -> Object;
}

// #[wasm_bindgen]
// #[derive(Default)]
// pub struct Imports;

// impl Imports {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// #[wasm_bindgen]
// impl Imports {
//     pub fn api_get_contract_address() -> i64 { 0 }
//     pub fn api_get_sender_address() -> i64 { 0 }
//     pub fn api_get_timestamp() -> i64 { 0 }

//     pub fn api_get_i32(_index: i32) -> i32 { 0 }
//     pub fn api_get_i64(_index: i32) -> i64 { 0 }
//     pub fn api_get_f32(_index: i32) -> f32 { 0.0 }
//     pub fn api_get_f64(_index: i32) -> f64 { 0.0 }

//     pub fn api_has_mapping(_index: i32, _key: i64) -> i32 { 0 }
//     pub fn api_get_mapping(_index: i32, _key: i64) -> i64 { 0 }

//     pub fn api_set_i32(_index: i32, _value: i32) -> () {}

//     pub fn api_set_i64(_index: i32, _value: i64) -> () {}

//     pub fn api_set_f32(_index: i32, _value: f32) -> () {}

//     pub fn api_set_f64(_index: i32, _value: f64) -> () {}

//     pub fn api_set_mapping(_index: i32, _key: i64, _value: i64) -> () {}
// }
