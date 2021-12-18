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

#[wasm_bindgen]
pub fn shared_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
#[repr(C)]
pub struct RawSlice {
    pub sptr: u32,
    pub len: u32,
}

#[derive(Clone, JsMetadata)]
#[repr(C)]
struct Edge {
    x: i32,
    y: i32,
    set: bool,
}

#[wasm_bindgen]
pub enum MetadataType {
    Struct,
    Enum,
    EnumWithFields,
}

#[wasm_bindgen]
pub struct Metadata {
    pub size: u32,
    pub ty: MetadataType,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn Edge_metadata() -> JsValue {
    JsValue::from_serde(&[
        MemoryField {
            name: String::from("x"),
            offset: offset_of!(Edge, x) as _,
            size: 4,
            ty: String::from("i32"),
        },
        MemoryField {
            name: String::from("y"),
            offset: offset_of!(Edge, y) as _,
            size: 4,
            ty: String::from("i32"),
        },
        MemoryField {
            name: String::from("set"),
            offset: offset_of!(Edge, set) as _,
            size: 1,
            ty: String::from("bool"),
        },
    ])
    .unwrap()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn Edge_size() -> u32 {
    std::mem::size_of::<Edge>() as _
}

#[wasm_bindgen]
#[repr(C)]
pub struct BoardData {
    edges: Vec<Edge>,
}

#[wasm_bindgen]
impl BoardData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        BoardData {
            edges: vec![
                Edge {
                    x: 0,
                    y: 0,
                    set: false,
                };
                100000
            ],
        }
    }

    pub fn raw(&self) -> RawSlice {
        RawSlice {
            sptr: self.edges.as_ptr() as _,
            len: self.edges.len() as _,
        }
    }
}

#[wasm_bindgen]
pub fn update_edges(board: &mut BoardData) {
    (0..100000i32).into_iter().for_each(|i| {
        board.edges[i as usize] = Edge {
            x: i + 10,
            y: i + 1,
            set: i % 2 == 0,
        };
    });
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn __wbg_edgeupdate_free() {}
