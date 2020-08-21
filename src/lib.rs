use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(x:i32, y:i32) -> i32 {
    x+y
}

#[wasm_bindgen]
pub struct STest {
    a: Vec<JsValue>,
}
#[wasm_bindgen]
impl STest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        STest { a: Vec::with_capacity(1000000)}
    }
    pub fn add(&mut self, v: JsValue)  {
        self.a.push(v);
    }
    pub fn pop(&mut self) {
        self.a.pop();
    }

}
