use fueros_derive::JsEnum;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use fueros_derive::JsEnum;

#[derive(Serialize, Deserialize, JsEnum, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    Real { id: u8 },
    System,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsEnum)]
pub enum Player {
    Bot { username: String },
    User { username: String },
}
