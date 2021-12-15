extern crate proc_macro;

use {proc_macro::TokenStream, quote::quote};

pub fn js_enum_impl(input: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(input).unwrap();
    let self_ty = &*ast.self_ty;

    let methods = ast
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Method(method) = item {
                if let Some(syn::FnArg::Receiver(receiver)) = method
                    .sig
                    .inputs
                    .iter()
                    .find(|input| matches!(input, syn::FnArg::Receiver(_)))
                {
                    Some((method, receiver.mutability.is_some()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(method, is_mut)| {
            let params = method
                .sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let syn::FnArg::Typed(arg) = arg {
                        Some(arg)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let original_method = &method.sig.ident;
            let method_ident = heck::AsLowerCamelCase(method.sig.ident.to_string()).to_string();
            let method_ident = syn::Ident::new(&method_ident, proc_macro2::Span::call_site());
            let return_type = &method.sig.output;
            let fwd_args = params.iter().map(|param| &*param.pat).collect::<Vec<_>>();
            if is_mut {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #method_ident(obj: ::js_sys::Object, #(#params,)*) #return_type {
                        let mut this = (&*obj).into_serde::<Self>().unwrap();
                        let out = this.#original_method(#(#fwd_args,)*);
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
                        (&*obj).into_serde::<Self>().unwrap().#original_method(#(#fwd_args,)*)
                    }
                }
            }
        }).collect::<Vec<_>>();

    TokenStream::from(quote! {
        #ast

        #[wasm_bindgen]
        impl #self_ty {
            #(#methods)*
        }
    })
}
