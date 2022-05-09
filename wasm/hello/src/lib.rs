extern crate wasm_bindgen;
 
use wasm_bindgen::prelude::*;
 
#[wasm_bindgen] // 导出函数
pub fn is_odd(n: u32) -> bool {
    n % 2 == 1
}