use wasm_bindgen::prelude::wasm_bindgen;

use fueros_derive::JsEnum;

#[derive(Clone, Copy, PartialEq, Eq, JsEnum)]
pub enum PlayerId {
    Real(u8),
    System,
}

#[derive(JsEnum)]
pub enum Player {
    Bot { username: String },
    User { username: String },
}
