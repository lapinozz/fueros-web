use fueros_derive::{js_enum_impl, JsEnum};
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[derive(JsEnum)]
pub enum Edge {
    Set { player_idx: u32 },
    Unset,
}

#[js_enum_impl]
impl Edge {
    pub fn is_set(&self) -> bool {
        matches!(self, Edge::Set { .. })
    }

    pub fn change_player(&mut self, new_player_idx: u32) {
        if let Edge::Set { player_idx } = self {
            *player_idx = new_player_idx;
        }
    }
}

#[wasm_bindgen()]
pub fn test() -> JsValue {
    // Your code goes here!
    JsValue::from_str("Hello my world!")
}
