use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: String);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn say_hi(name: String) -> String {
    format!("Hello, {}", name)
}

#[wasm_bindgen]
pub fn say_hi_and_alert(name: String) {
    alert(say_hi(name));
}

#[wasm_bindgen]
pub fn say_hi_to_console(name: String) {
    log(say_hi(name));
}
