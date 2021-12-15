use serde::{Deserialize, Serialize};
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
    describe::WasmDescribe,
    JsValue,
};

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

// TODO: Derive macro for this module, and the Wasm traits
mod wasm_impl {
    use super::PlayerId;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    #[allow(non_snake_case)]
    impl PlayerId {
        #[wasm_bindgen(js_name = Real)]
        pub fn _real(x: u8) -> Self {
            PlayerId::Real(x)
        }

        #[wasm_bindgen(js_name = System)]
        pub fn _system() -> Self {
            PlayerId::System
        }
    }
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
