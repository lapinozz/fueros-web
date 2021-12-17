use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

extern crate proc_macro;

use quote::quote;

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let name_str = name.to_string();
    let name_len = name_str.len() as u32;
    let name_chars = name_str.chars().map(|c| c as u32);

    let accessors = generate_accessors(&ast);
    let metadata = generate_metadata(&ast);
    let variant_accessor = generate_variant_accessor(&ast);

    let free = quote::format_ident!("__wbg_{}_free", name_str.to_lowercase());

    TokenStream::from(quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        impl #name {
            #[wasm_bindgen(constructor)]
            pub fn new(js: wasm_bindgen::JsValue) -> Self {
                js.into_serde().expect(&format!(
                    "{}.new(): invalid enum object format",
                    #name_str
                ))
            }

            #(#accessors)*

            #metadata

            #variant_accessor
        }

        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[no_mangle]
        #[doc(hidden)]
        #[allow(clippy::all)]
        pub unsafe extern "C" fn #free(ptr: u32) {
            drop(<#name as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr));
        }

        impl wasm_bindgen::describe::WasmDescribe for #name {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(RUST_STRUCT);
                inform(#name_len);
                #(inform(#name_chars);)*
            }
        }

        impl wasm_bindgen::convert::IntoWasmAbi for #name {
            type Abi = u32;

            fn into_abi(self) -> u32 {
                use wasm_bindgen::__rt::std::boxed::Box;
                use wasm_bindgen::__rt::WasmRefCell;
                Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
            }
        }

        impl wasm_bindgen::convert::FromWasmAbi for #name {
            type Abi = u32;

            unsafe fn from_abi(js: u32) -> Self {
                use wasm_bindgen::__rt::std::boxed::Box;
                use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};

                let ptr = js as *mut WasmRefCell<Self>;
                assert_not_null(ptr);
                let js = Box::from_raw(ptr);
                (*js).borrow_mut(); // make sure no one's borrowing
                js.into_inner()
            }
        }

        impl wasm_bindgen::convert::OptionIntoWasmAbi for #name {
            fn none() -> Self::Abi {
                use wasm_bindgen::convert::IntoWasmAbi;
                wasm_bindgen::JsValue::null().into_abi()
            }
        }

        impl wasm_bindgen::convert::OptionFromWasmAbi for #name {
            fn is_none(abi: &Self::Abi) -> bool {
                use wasm_bindgen::convert::FromWasmAbi;
                unsafe { wasm_bindgen::JsValue::from_abi(*abi) }.is_null()
            }
        }

        impl wasm_bindgen::convert::RefFromWasmAbi for #name {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::Ref<'static, Self>;

            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Self>;
                wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow()
            }
        }

        impl wasm_bindgen::convert::RefMutFromWasmAbi for #name {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RefMut<'static, Self>;

            unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Self>;
                wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow_mut()
            }
        }
    })
}

fn generate_accessors(ast: &syn::DeriveInput) -> Vec<TokenStream2> {
    if let syn::Data::Enum(data) = &ast.data {
        data.variants
            .iter()
            .map(|variant| {
                variant.fields.iter().enumerate().map(|(i, field)| {
                    let field_name = field
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or_else(|| i.to_string());
                    let field_name: TokenStream2 = field_name.parse().unwrap();

                    let setter = quote::format_ident!(
                        "__{}_set_{}",
                        &variant.ident,
                        camel_case(&field_name)
                    );

                    let getter = quote::format_ident!(
                        "__{}_get_{}",
                        &variant.ident,
                        camel_case(&field_name)
                    );

                    let field_type = &field.ty;
                    let variant_name = &variant.ident;

                    quote! {
                        pub fn #setter(&mut self, __value: #field_type) {
                            if let Self::#variant_name { #field_name: __cur_value, .. } = self {
                                *__cur_value = __value;
                            } else {
                                panic!("attempted to set field for a non-current variant");
                            }
                        }

                        pub fn #getter(&self) -> #field_type {
                            if let Self::#variant_name { #field_name: __cur_value, .. } = self {
                                __cur_value.clone()
                            } else {
                                panic!("attempted to get field for a non-current variant");
                            }
                        }
                    }
                })
            })
            .flatten()
            .collect()
    } else {
        panic!("JsEnum only operates on enums")
    }
}

fn generate_metadata(ast: &syn::DeriveInput) -> TokenStream2 {
    if let syn::Data::Enum(data) = &ast.data {
        let insert_variant = data.variants.iter().map(|variant| {
            let variant_str = variant.ident.to_string();
            let field_names = variant.fields.iter().enumerate().map(|(i, field)| {
                let mut field_name = camel_case(
                    &field
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or_else(|| i.to_string()),
                );

                if field.attrs.iter().any(|attr| {
                    attr.path
                        .get_ident()
                        .map(|ident| &ident.to_string() == "nested")
                        .unwrap_or(false)
                }) {
                    field_name = format!("@{}", field_name);
                }

                field_name
            });
            quote! {
                __out_metadata.insert(String::from(#variant_str), [
                    #(String::from(#field_names),)*
                ].to_vec());
            }
        });

        quote! {
            pub fn __JsEnum_Metadata() -> wasm_bindgen::JsValue {
                let mut __out_metadata: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                #(#insert_variant)*
                wasm_bindgen::JsValue::from_serde(&__out_metadata).unwrap()
            }
        }
    } else {
        panic!("JsEnum only operates on enums")
    }
}

fn generate_variant_accessor(ast: &syn::DeriveInput) -> TokenStream2 {
    if let syn::Data::Enum(data) = &ast.data {
        let match_cases = data.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let variant_str = variant.ident.to_string();
            quote! {
                Self::#variant_name { .. } => String::from(#variant_str)
            }
        });

        quote! {
            pub fn __getVariant(&self) -> String {
                match self {
                    #(#match_cases,)*
                }
            }
        }
    } else {
        panic!("JsEnum only operates on enums")
    }
}

fn camel_case(s: impl ToString) -> String {
    heck::AsLowerCamelCase(s.to_string()).to_string()
}
