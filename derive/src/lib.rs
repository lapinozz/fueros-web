mod js_enum_impl;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn js_enum_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    js_enum_impl::js_enum_impl(input)
}
