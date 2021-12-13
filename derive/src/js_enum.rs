extern crate proc_macro;

use {proc_macro::TokenStream, quote::quote};

pub fn js_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    struct Variant {
        name: String,
        in_fields: Vec<syn::Ident>,
        out_fields: Vec<(syn::Ident, syn::Type)>,
    }

    let mut variants = Vec::new();

    if let syn::Data::Enum(enum_data) = &ast.data {
        let ident = &ast.ident;
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;
            variants.push(Variant {
                name: variant_ident.to_string(),
                in_fields: variant
                    .fields
                    .iter()
                    .map(|x| x.ident.clone().unwrap())
                    .collect::<Vec<_>>(),
                out_fields: variant
                    .fields
                    .iter()
                    .map(|field| {
                        (
                            quote::format_ident!(
                                "{}_{}",
                                variant_ident,
                                field.ident.clone().unwrap()
                            ),
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
                    pub #ident: Option<#ty>
                }
            });

        let from_match_cases = variants
            .iter()
            .map(|x| {
                let variant_ident = quote::format_ident!("{}", x.name);
                let full_variant_ident = quote! { #ident::#variant_ident };
                let field_names = x.in_fields.iter().map(|x| x.clone()).collect::<Vec<_>>();
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
                quote! {
                    #full_variant_ident {
                        #(#field_names,)*
                        ..
                    } => {
                        #out_ident {
                            variant: std::stringify!(#variant_ident),
                            #(#fields,)*
                            ..Default::default()
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        {
            quote! {
                #[derive(Default)]
                pub struct #out_ident {
                    pub variant: &'static str,
                    #(#out_fields,)*
                }

                impl From<#ident> for #out_ident {
                    fn from(e: #ident) -> #out_ident {
                        match e {
                            #(#from_match_cases)*
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
