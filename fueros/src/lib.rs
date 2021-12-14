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

#[wasm_bindgen()]
pub fn test() -> JsValue {
    // Your code goes here!
    JsValue::from_str("Hello my world!")
}

#[cfg(test)]
mod tests {
    use fueros_derive::JsEnum;
    use wasm_bindgen::prelude::*;

    #[test]
    fn test_js_enum() {
        #[derive(JsEnum, Clone, Debug, PartialEq)]
        enum TestEnum {
            Hello { number: u32, string: String },
            World { array: Vec<u32> },
        }

        let original = TestEnum::Hello {
            number: 1,
            string: String::from("bruh"),
        };

        let js: JsTestEnum = original.clone().into();

        assert_eq!(js.variant, "Hello");
        assert_eq!(js.Hello_number, Some(1));
        assert_eq!(js.Hello_string, Some("bruh".to_string()));

        let converted: TestEnum = js.into();
        assert_eq!(converted, original);
    }
}
