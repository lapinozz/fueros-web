use proc_macro::TokenStream;
use syn::{Ident, ImplItem, ImplItemMethod, PatType, ReturnType};

extern crate proc_macro;

use quote::quote;

pub fn js_enum_impl(input: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(input).unwrap();
    let self_ty = &*ast.self_ty;

    let methods = ast
        .items
        .iter()
        .filter_map(Method::from_impl_item)
        .map(|m| m.get_js_compatible_version())
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #ast

        #[wasm_bindgen]
        impl #self_ty {
            #(#methods)*
        }
    })
}

/// A function of an enum that has `&self` or `&mut self` as the first parameter.
/// It is meant to be used to generate a version of itself which uses a JS object instead of
/// the original enum.
struct Method<'i> {
    inner: &'i ImplItemMethod,
    is_mut: bool,
}

impl<'i> Method<'i> {
    /// Obtains an instance of `Method` from an implementation item, if it is valid.
    pub fn from_impl_item(item: &'i ImplItem) -> Option<Self> {
        if let syn::ImplItem::Method(method) = item {
            method.sig.inputs.iter().find_map(|input| {
                if let syn::FnArg::Receiver(receiver) = input {
                    Some(Method {
                        inner: method,
                        is_mut: receiver.mutability.is_some(),
                    })
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    /// Returns the identifier for the original enum function this method came from (i.e. Rust-side)
    pub fn original_ident(&self) -> &Ident {
        &self.inner.sig.ident
    }

    /// Returns the identifier for the to-be generated function that takes a JS object (i.e. JS-side)
    pub fn js_ident(&self) -> Ident {
        let method_name = heck::AsLowerCamelCase(self.original_ident().to_string()).to_string();
        syn::Ident::new(&method_name, proc_macro2::Span::call_site())
    }

    /// Returns the original function's return or output type.
    pub fn return_type(&self) -> &ReturnType {
        &self.inner.sig.output
    }

    /// An iterator returning the parameters of this function, excluding the typeless `self`.
    pub fn params(&self) -> impl std::iter::Iterator<Item = &'i PatType> + Clone {
        self.inner.sig.inputs.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(arg) = arg {
                Some(arg)
            } else {
                None
            }
        })
    }

    /// Generate a version of this function that takes in a JS object instead of the original enum.
    pub fn get_js_compatible_version(&self) -> proc_macro2::TokenStream {
        let original_method_ident = self.original_ident();
        let method_ident = self.js_ident();
        let return_type = self.return_type();
        let params = self.params();
        let fwd_args = params.clone().map(|param| &*param.pat);

        if self.is_mut {
            quote! {
                #[allow(non_snake_case)]
                pub fn #method_ident(obj: ::js_sys::Object, #(#params,)*) #return_type {
                    let mut this = (&*obj).into_serde::<Self>().unwrap();
                    let out = this.#original_method_ident(#(#fwd_args,)*);
                    ::js_sys::Object::assign(
                        &obj,
                        &::js_sys::Object::from(::wasm_bindgen::JsValue::from_serde(&this).unwrap()),
                    );
                    out
                }
            }
        } else {
            quote! {
                #[allow(non_snake_case)]
                pub fn #method_ident(obj: ::js_sys::Object, #(#params,)*) #return_type {
                    (&*obj).into_serde::<Self>().unwrap().#original_method_ident(#(#fwd_args,)*)
                }
            }
        }
    }
}
