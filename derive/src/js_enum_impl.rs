extern crate proc_macro;

use {proc_macro::TokenStream, quote::quote};

pub fn js_enum_impl(input: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(input).unwrap();

    let ident = if let syn::Type::Path(path) = &*ast.self_ty {
        path.path.get_ident().unwrap()
    } else {
        panic!("this ain't no enum impl")
    };

    let out_ident = quote::format_ident!("Js{}", ident);
    let out_methods = ast
        .items
        .iter()
        .filter_map(|x| {
            if let syn::ImplItem::Method(method) = x {
                let vis = &method.vis;
                let sig = &method.sig;
                let fn_name = &sig.ident;

                let is_mut = method
                    .sig
                    .inputs
                    .iter()
                    .find_map(|arg| {
                        if let syn::FnArg::Receiver(arg) = arg {
                            Some(arg)
                        } else {
                            None
                        }
                    })?
                    .mutability
                    .is_some();

                let non_self_args = sig
                    .inputs
                    .iter()
                    .filter_map(|arg| {
                        if let syn::FnArg::Typed(arg) = arg {
                            Some(arg.pat.as_ref())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Some(if is_mut {
                    quote! {
                        #vis #sig {
                            let mut this: #ident = std::mem::take(self).into();
                            let out = this.#fn_name(#(#non_self_args,)*);
                            *self = this.into();
                            out
                        }
                    }
                } else {
                    quote! {
                        #vis #sig {
                            let this: #ident = self.clone().into();
                            this.#fn_name(#(#non_self_args,)*)
                        }
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    {
        quote! {
            #ast

            #[allow(non_snake_case)]
            #[wasm_bindgen]
            impl #out_ident {
                #(#out_methods)*
            }
        }
    }
    .into()
}
