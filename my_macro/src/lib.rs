extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(GettersForFieldNames)]
pub fn getters_for_field_names_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields: Vec<_> = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .map(|f| &f.ident)
                .collect(),
            _ => panic!("This macro only works with named fields"),
        },
        _ => panic!("This macro only works with structs"),
    };

    let getters: Vec<_> = fields
        .iter()
        .map(|field| {
            let getter_name = format!("get_{}", field.as_ref().unwrap());
            let getter_ident = syn::Ident::new(&getter_name, field.as_ref().unwrap().span());
            quote! {
                pub fn #getter_ident(&self) -> &'static str {
                    stringify!(#field)
                }
            }
        })
        .collect();

    let gen = quote! {
        impl #name {
            #(#getters)*
        }
    };

    gen.into()
}