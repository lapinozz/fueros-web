mod js_enum;

use proc_macro::TokenStream;

#[proc_macro_derive(JsEnum, attributes(nested))]
pub fn js_enum(input: TokenStream) -> TokenStream {
    js_enum::js_enum(input)
}
