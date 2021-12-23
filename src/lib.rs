pub mod board;
pub mod game;
pub mod player;
pub mod util;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

use fueros_derive::JsEnum;

#[derive(Serialize, Deserialize, JsEnum, Clone, Copy)]
pub enum Values {
    First { value: i32 },
    Second,
    Third (u32),
    Fourth (u32, u8),
    Fifth (u32, u8, u16),
}

#[wasm_bindgen]
impl Values {}

#[wasm_bindgen]
pub fn testValuesFirst() -> Values {
    Values::First{value: 1}
}

#[wasm_bindgen]
pub fn testValuesSecond() -> Values {
    Values::Second
}

#[wasm_bindgen]
pub fn testValuesThird() -> Values {
    Values::Third(1)
}

#[wasm_bindgen]
pub fn testValuesFourth() -> Values {
    Values::Fourth(1, 2)
}

#[wasm_bindgen]
pub fn testValuesFifth() -> Values {
    Values::Fifth(1, 2, 3)
}

/// Marker trait implemented by `derive(JsEnum)`. Ignore. Ignore. Do not look at the moon. Ignore. Ignore.
pub trait JsEnum {}

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
