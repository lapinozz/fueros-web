use fueros_derive::JsEnum;
use serde::{Deserialize, Serialize};

trait JsEnum {}

#[test]
fn test_js_enum() {
    #[derive(Serialize, Deserialize, JsEnum)]
    pub enum Enum {
        V1 { a: i32, snake_case: u32 },
        V2(i32, u32),
        V3,
    }

    let mut e = Enum::V1 {
        a: 0,
        snake_case: 1,
    };

    e.__V1_set_a(2);

    assert_eq!(e.__V1_get_a(), 2);
    assert_eq!(e.__V1_get_snakeCase(), 1);
    assert_eq!(&e.__getVariant(), "V1");

    e = Enum::V2(0, 1);

    e.__V2_set_0(2);

    assert_eq!(e.__V2_get_0(), 2);
    assert_eq!(e.__V2_get_1(), 1);
    assert_eq!(&e.__getVariant(), "V2");

    e = Enum::V3;

    assert_eq!(&e.__getVariant(), "V3");
}
