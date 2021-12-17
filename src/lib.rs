use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct EdgeUpdate {
    pub x: i32,
    pub y: i32,
    pub set: bool,
}

#[wasm_bindgen]
struct Message {}

#[wasm_bindgen]
pub struct Callbacks {
    set_edges: js_sys::Function,
}

#[wasm_bindgen]
impl Callbacks {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Callbacks {
            set_edges: js_sys::Function::default(),
        }
    }

    pub fn set_edges(&mut self, f: js_sys::Function) {
        self.set_edges = f;
    }
}

#[wasm_bindgen]
pub fn run_game(callbacks: Callbacks) {
    let game = Game {
        set_edges: Box::new(move |edges: &[EdgeUpdate]| {
            callbacks
                .set_edges
                .call1(
                    &JsValue::NULL,
                    &edges
                        .into_iter()
                        .cloned()
                        .map(JsValue::from)
                        .collect::<js_sys::Array>(),
                )
                .unwrap();
        }),
    };

    game.run();
}

pub struct Game {
    set_edges: Box<dyn Fn(&[EdgeUpdate])>,
}

impl Game {
    pub fn run(self) {
        let updates = (0..100000)
            .into_iter()
            .map(|i| EdgeUpdate {
                x: i,
                y: i,
                set: false,
            })
            .collect::<Vec<_>>();
        (*self.set_edges)(&updates);
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}
