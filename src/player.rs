use fueros_derive::JsEnum;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsEnum)]
pub enum PlayerId {
    Real(u8),
    System,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsEnum)]
pub enum Player {
    Bot { username: String },
    User { username: String },
}
