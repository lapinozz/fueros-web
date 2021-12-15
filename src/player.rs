use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, describe::WasmDescribe, convert::{IntoWasmAbi, FromWasmAbi}, JsValue};


#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerId {
    Real(u8),
    System,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Bot { username: String },
    User { username: String },
}

impl WasmDescribe for PlayerId {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for PlayerId {
    type Abi = u32;

    fn into_abi(self) -> Self::Abi {
        JsValue::from_serde(&self).unwrap().into_abi()
    }
}

impl FromWasmAbi for PlayerId {
    type Abi = u32;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        JsValue::from_abi(js)
            .into_serde()
            .expect("Couldn't obtain valid PlayerId from JS object representation")
    }
}

