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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_enum() {
        #[derive(JsEnum, Clone, Debug, PartialEq)]
        enum TestEnum {
            Hello { number: u32, string: String },
            World { array: Vec<u32> },
        }

        #[js_enum_impl(TestEnum)]
        impl TestEnum {
            pub fn add_if_hello(&mut self, n: u32) {
                match self {
                    TestEnum::Hello { number, .. } => *number += n,
                    _ => {}
                }
            }

            pub fn is_world(&self) -> bool {
                matches!(self, TestEnum::World { .. })
            }
        }

        let original = TestEnum::Hello {
            number: 1,
            string: String::from("bruh"),
        };

        let mut js: JsTestEnum = original.clone().into();

        assert_eq!(js.variant, "Hello");
        assert_eq!(js.Hello_number, Some(1));
        assert_eq!(js.Hello_string, Some("bruh".to_string()));

        let converted: TestEnum = js.clone().into();
        assert_eq!(converted, original);

        js.add_if_hello(5);
        assert_eq!(js.Hello_number, Some(6));
        assert_eq!(js.is_world(), false);
    }

    #[test]
    fn test_unnamed_enum() {
        #[derive(JsEnum, Clone)]
        enum TestEnum {
            Hello(u32, String),
            World(Vec<u32>),
        }

        let original = TestEnum::Hello(1, String::from("bruh"));

        let js: JsTestEnum = original.clone().into();

        assert_eq!(js.variant, "Hello");
        assert_eq!(js.Hello_0, Some(1));
        assert_eq!(js.Hello_1, Some("bruh".to_string()));
    }

    #[test]
    fn test_accessor() {
        #[derive(JsEnum)]
        enum TestEnum {
            Hello { number: u32, string: String },
        }

        let original = TestEnum::Hello {
            number: 5,
            string: String::from("bruh"),
        };

        let js: JsTestEnum = original.into();

        assert_eq!(js.Hello_number(), Some(5));
        assert_eq!(js.Hello_string(), Some(String::from("bruh")));
    }
}
