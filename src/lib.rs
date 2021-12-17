use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EdgeUpdate {
    pub x: i32,
    pub y: i32,
    pub set: bool,
}

#[wasm_bindgen]
pub struct Callbacks {
    set_edges: js_sys::Function,
}

#[wasm_bindgen]
impl Callbacks {
    #[wasm_bindgen(setter)]
    pub fn set_edges(&mut self, f: js_sys::Function) {
        self.set_edges = f;
    }
}

#[wasm_bindgen]
pub fn run_game(callbacks: Callbacks) {
    let game = Game {
        set_edges: Box::new(move |edges: &[EdgeUpdate]| {
            use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
            callbacks.set_edges.call1(
                &JsValue::NULL,
                &edges
                    .into_iter()
                    .map(JsValue::from)
                    .collect::<js_sys::Array>(),
            );
        }),
    };
}

pub struct Game {
    set_edges: Box<dyn Fn(&[EdgeUpdate])>,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}
