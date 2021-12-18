use memoffset::offset_of;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct MemoryField {
    pub name: String,
    pub offset: u32,
    pub size: u32,
    pub ty: String,
}

#[repr(C, packed)]
pub struct EdgeUpdate {
    pub x: i32,
    pub y: i32,
    pub set: bool,
}

#[wasm_bindgen]
impl EdgeUpdate {
    pub fn metadata() -> JsValue {
        JsValue::from_serde(&[
            MemoryField {
                name: String::from("x"),
                offset: offset_of!(EdgeUpdate, x) as _,
                size: 4,
                ty: String::from("i32"),
            },
            MemoryField {
                name: String::from("y"),
                offset: offset_of!(EdgeUpdate, y) as _,
                size: 4,
                ty: String::from("i32"),
            },
            MemoryField {
                name: String::from("set"),
                offset: offset_of!(EdgeUpdate, set) as _,
                size: 1,
                ty: String::from("bool"),
            },
        ])
        .unwrap()
    }

    pub fn size() -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}

#[wasm_bindgen]
pub fn shared_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
struct RawSlice {
    pub sptr: u32,
    pub len: u32,
}

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
                    &JsValue::from(RawSlice {
                        sptr: edges.as_ptr() as u32,
                        len: edges.len() as u32,
                    }),
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
                x: i + 10,
                y: i + 1,
                set: i % 2 == 0,
            })
            .collect::<Vec<_>>();
        (*self.set_edges)(&*updates);
        std::mem::forget(updates); // leak to give JS time to read (temporary fix obviously)
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}


#[wasm_bindgen]
pub fn __wbg_edgeupdate_free()
{

}