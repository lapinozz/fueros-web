use fueros_derive::js_enum_impl;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello my world!"));

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub enum Edge {
    #[serde(rename_all = "camelCase")]
    Set { player_idx: u32 },
    #[serde(rename_all = "camelCase")]
    Unset,
}

#[js_enum_impl]
impl Edge {
    pub fn is_set(&self) -> bool {
        matches!(self, Edge::Set { .. })
    }

    pub fn change_player(&mut self, new_player_idx: u32) -> u32 {
        if let Edge::Set { player_idx } = self {
            let prev = *player_idx;
            *player_idx = new_player_idx;
            prev
        } else {
            0
        }
    }
}

#[wasm_bindgen()]
pub fn test() -> JsValue {
    // Your code goes here!
    JsValue::from_str("Hello my world!")
}
