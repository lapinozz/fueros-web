use proc_macro::TokenStream;
use syn::{Ident, ImplItem, ImplItemMethod, PatType, ReturnType};

extern crate proc_macro;

use quote::quote;

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(input).unwrap();

    todo!()
}
