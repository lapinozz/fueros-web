extern crate proc_macro;

use {proc_macro::TokenStream, proc_macro2::TokenStream as TokenStream2, quote::quote};

enum VariantForm {
    Unit,
    Unnamed,
    Named,
}

struct Variant {
    name: String,
    in_fields: Vec<syn::Ident>,
    out_fields: Vec<(syn::Ident, syn::Type)>,
    form: VariantForm,
}

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let syn::Data::Enum(enum_data) = &ast.data {
        let ident = &ast.ident;

        let variants = generate_variant_data(enum_data);

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

        let from_match_cases = from_enum_match_cases(&variants, ident, &out_ident);
        let into_match_cases = from_js_match_cases(&variants, ident);
        let accessors = generate_accessors(&variants);
        let variant_ctors = generate_variant_ctors(&variants);

        {
            quote! {
                #[allow(non_snake_case)]
                #[wasm_bindgen]
                #[derive(Default, Clone)]
                pub struct #out_ident {
                    variant: String,
                    #(#out_fields,)*
                }

                #[allow(non_snake_case)]
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

                #[allow(non_snake_case)]
                impl From<#ident> for #out_ident {
                    fn from(e: #ident) -> #out_ident {
                        match e {
                            #(#from_match_cases)*
                        }
                    }
                }

                #[allow(non_snake_case)]
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

fn generate_variant_ctors(variants: &[Variant]) -> Vec<TokenStream2> {
    variants
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
        .collect()
}

fn generate_variant_data(input: &syn::DataEnum) -> Vec<Variant> {
    input
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            Variant {
                name: variant_ident.to_string(),
                in_fields: variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
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
                form: match &variant.fields {
                    syn::Fields::Unit => VariantForm::Unit,
                    syn::Fields::Unnamed(_) => VariantForm::Unnamed,
                    syn::Fields::Named(_) => VariantForm::Named,
                },
            }
        })
        .collect()
}

fn from_enum_match_cases(
    variants: &[Variant],
    ident: &syn::Ident,
    out_ident: &syn::Ident,
) -> Vec<TokenStream2> {
    variants
        .iter()
        .map(|variant| {
            let variant_ident = quote::format_ident!("{}", variant.name);
            let full_variant_ident = quote! { #ident::#variant_ident };
            let field_names = &variant.in_fields;
            let fields = variant
                .in_fields
                .iter()
                .zip(variant.out_fields.iter())
                .map(|(in_field, (out_field, _))| {
                    quote! {
                        #out_field: Some(#in_field)
                    }
                })
                .collect::<Vec<_>>();
            match &variant.form {
                VariantForm::Unit => quote! {
                    #full_variant_ident => {
                        Self {
                            variant: std::stringify!(#variant_ident).to_string(),
                            ..Default::default()
                        }
                    }
                },
                VariantForm::Unnamed => quote! {
                    #full_variant_ident(#(#field_names,)*) => {
                        #out_ident {
                            variant: std::stringify!(#variant_ident).to_string(),
                            #(#fields,)*
                            ..Default::default()
                        }
                    }
                },
                VariantForm::Named => quote! {
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
                },
            }
        })
        .collect()
}

fn from_js_match_cases(variants: &[Variant], ident: &syn::Ident) -> Vec<TokenStream2> {
    variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.name;
            let variant_ident = quote::format_ident!("{}", variant.name);
            let fields = variant
                .in_fields
                .iter()
                .zip(variant.out_fields.iter())
                .map(|(in_field, (out_field, _))| match &variant.form {
                    VariantForm::Unit => Default::default(),
                    VariantForm::Unnamed => quote! {
                        e.#out_field.unwrap()
                    },
                    VariantForm::Named => quote! {
                        #in_field: e.#out_field.unwrap()
                    },
                })
                .collect::<Vec<_>>();
            match &variant.form {
                VariantForm::Unit => quote! {
                    #variant_name => {
                        #ident::#variant_ident
                    }
                },
                VariantForm::Unnamed => quote! {
                    #variant_name => {
                        #ident::#variant_ident(#(#fields,)*)
                    }
                },
                VariantForm::Named => quote! {
                    #variant_name => {
                        #ident::#variant_ident {
                            #(#fields,)*
                        }
                    }
                },
            }
        })
        .collect()
}

fn generate_accessors(variants: &[Variant]) -> Vec<TokenStream2> {
    variants
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
        .collect()
}
