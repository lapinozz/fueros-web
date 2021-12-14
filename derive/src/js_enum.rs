extern crate proc_macro;

use {proc_macro::TokenStream, quote::quote};

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let syn::Data::Enum(enum_data) = &ast.data {
        struct Variant {
            name: String,
            in_fields: Vec<syn::Ident>,
            out_fields: Vec<(syn::Ident, syn::Type)>,
        }

        let mut variants = Vec::new();
        let mut is_unnamed = false;

        let ident = &ast.ident;
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;
            variants.push(Variant {
                name: variant_ident.to_string(),
                in_fields: variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if x.ident.is_none() {
                            is_unnamed = true;
                        }
                        x.ident
                            .clone()
                            .unwrap_or_else(|| quote::format_ident!("x{}", i))
                    })
                    .collect::<Vec<_>>(),
                out_fields: variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        (
                            field
                                .ident
                                .as_ref()
                                .map(|x| quote::format_ident!("{}_{}", variant_ident, x))
                                .unwrap_or_else(|| quote::format_ident!("{}_{}", variant_ident, i)),
                            field.ty.clone(),
                        )
                    })
                    .collect(),
            });
        }

        let out_ident = quote::format_ident!("Js{}", ident);
        let out_fields = variants
            .iter()
            .map(|x| &x.out_fields)
            .flatten()
            .map(|(ident, ty)| {
                quote! {
                    #ident: Option<#ty>
                }
            });

        let from_match_cases = variants
            .iter()
            .map(|x| {
                let variant_ident = quote::format_ident!("{}", x.name);
                let full_variant_ident = quote! { #ident::#variant_ident };
                let field_names = &x.in_fields;
                let fields = x
                    .in_fields
                    .iter()
                    .zip(x.out_fields.iter())
                    .map(|(in_field, (out_field, _))| {
                        quote! {
                            #out_field: Some(#in_field)
                        }
                    })
                    .collect::<Vec<_>>();
                if is_unnamed {
                    quote! {
                        #full_variant_ident(#(#field_names,)*) => {
                            #out_ident {
                                variant: std::stringify!(#variant_ident).to_string(),
                                #(#fields,)*
                                ..Default::default()
                            }
                        }
                    }
                } else {
                    quote! {
                        #full_variant_ident {
                            #(#field_names,)*
                            ..
                        } => {
                            #out_ident {
                                variant: std::stringify!(#variant_ident).to_string(),
                                #(#fields,)*
                                ..Default::default()
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let into_match_cases = variants
            .iter()
            .map(|x| {
                let variant_name = &x.name;
                let variant_ident = quote::format_ident!("{}", x.name);
                let fields = x
                    .in_fields
                    .iter()
                    .zip(x.out_fields.iter())
                    .map(|(in_field, (out_field, _))| {
                        if is_unnamed {
                            quote! {
                                e.#out_field.unwrap()
                            }
                        } else {
                            quote! {
                                #in_field: e.#out_field.unwrap()
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                if is_unnamed {
                    quote! {
                        #variant_name => {
                            #ident::#variant_ident(#(#fields,)*)
                        }
                    }
                } else {
                    quote! {
                        #variant_name => {
                            #ident::#variant_ident {
                                #(#fields,)*
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let accessors = variants
            .iter()
            .map(|x| {
                x.out_fields.iter().map(|(out_field, ty)| {
                    let setter = quote::format_ident!("set_{}", out_field);
                    quote! {
                        #[wasm_bindgen(getter)]
                        pub fn #out_field(&self) -> Option<#ty> {
                            self.#out_field.clone()
                        }

                        #[wasm_bindgen(setter)]
                        pub fn #setter(&mut self, x: Option<#ty>) {
                            self.#out_field = x;
                        }
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        let variant_ctors = variants
            .iter()
            .map(|x| {
                let variant_ident = quote::format_ident!("{}", x.name);
                let params = x
                    .out_fields
                    .iter()
                    .map(|(out_field, ty)| {
                        quote! {
                            #out_field: #ty
                        }
                    })
                    .collect::<Vec<_>>();
                let fields = x
                    .out_fields
                    .iter()
                    .map(|(out_field, _)| {
                        quote! {
                            #out_field: Some(#out_field)
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    pub fn #variant_ident(#(#params,)*) -> Self {
                        Self {
                            variant: std::stringify!(#variant_ident).to_string(),
                            #(#fields,)*
                            ..Default::default()
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        {
            quote! {
                #[wasm_bindgen]
                #[derive(Default, Clone)]
                pub struct #out_ident {
                    variant: String,
                    #(#out_fields,)*
                }

                #[wasm_bindgen]
                impl #out_ident {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> Self {
                        Default::default()
                    }

                    #(#variant_ctors)*

                    #[wasm_bindgen(getter)]
                    pub fn variant(&self) -> String {
                        self.variant.clone()
                    }

                    #[wasm_bindgen(setter)]
                    pub fn set_variant(&mut self, variant: String) {
                        self.variant = variant;
                    }

                    #(#accessors)*
                }

                impl From<#ident> for #out_ident {
                    fn from(e: #ident) -> #out_ident {
                        match e {
                            #(#from_match_cases)*
                        }
                    }
                }

                impl From<#out_ident> for #ident {
                    fn from(e: #out_ident) -> #ident {
                        match &e.variant[..] {
                            #(#into_match_cases)*
                            _ => panic!("invalid JsEnum variant '{}'", &e.variant),
                        }
                    }
                }
            }
        }
        .into()
    } else {
        panic!("JsEnum only works on enums bozo #getreal #getsmart #geteducated xDDD")
    }
}
