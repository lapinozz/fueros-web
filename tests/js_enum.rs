use fueros_derive::*;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_enum_conversion() {
        #[derive(JsEnum, Clone, Debug, PartialEq)]
        enum Enum {
            First { value: i32 },
            Second(i32),
            Third,
        }

        let first = Enum::First { value: 1 };
        let second = Enum::Second(1);
        let third = Enum::Third;

        assert_eq!(first.clone(), Enum::from(JsEnum::from(first)));
        assert_eq!(second.clone(), Enum::from(JsEnum::from(second)));
        assert_eq!(third.clone(), Enum::from(JsEnum::from(third)));
    }

    #[test]
    fn test_js_enum_methods() {
        #[derive(JsEnum)]
        enum Enum {
            Value(i32),
        }

        #[js_enum_impl]
        impl Enum {
            fn read(&self, n: i32) -> i32 {
                match self {
                    Enum::Value(x) => *x + n,
                }
            }

            fn mutate(&mut self, val: i32) -> &'static str {
                match self {
                    Enum::Value(x) => *x = val,
                }
                "valid return value"
            }
        }

        let e = Enum::Value(0);

        let mut js = JsEnum::from(e);

        assert_eq!(js.mutate(2), "valid return value");
        assert_eq!(js.read(3), 5);
    }

    #[test]
    fn test_js_enum_accessors() {
        #[derive(JsEnum)]
        enum Enum {
            First { value: i32 },
            Second { value: i32 },
        }

        let mut e1 = JsEnum::from(Enum::Second { value: 1 });
        e1.set_Second_value(Some(2));

        assert_eq!(e1.First_value(), None);
        assert_eq!(e1.Second_value(), Some(2));
    }
}
