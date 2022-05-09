extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen] // 导入js函数
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen] // 导出rust函数
pub fn is_odd(n: u32) -> bool {
    let s = n.to_string();
    web_sys::console::log_1(&s.into());
    n % 2 == 1
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    web_sys::console::log_1(&s.into());
    alert(&s);
}
