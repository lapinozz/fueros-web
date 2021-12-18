mod metadata;

use proc_macro::TokenStream;

#[proc_macro_derive(JsMetadata)]
pub fn js_metadata(input: TokenStream) -> TokenStream {
    metadata::metadata(input)
}
