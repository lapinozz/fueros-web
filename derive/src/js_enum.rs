use proc_macro::TokenStream;

extern crate proc_macro;

use quote::quote;

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let name_str = name.to_string();
    let name_len = name_str.len() as u32;
    let name_chars = name_str.chars().map(|c| c as u32);

    TokenStream::from(quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #name {
            #[wasm_bindgen(constructor)]
            pub fn new(js: wasm_bindgen::JsValue) -> Self {
                js.into_serde().expect(&format!(
                    "{}.new(): invalid enum object format",
                    #name_str
                ))
            }

            #[wasm_bindgen(getter)]
            pub fn variant(&self) -> wasm_bindgen::JsValue {
                wasm_bindgen::JsValue::from_serde(self).unwrap()
            }
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
