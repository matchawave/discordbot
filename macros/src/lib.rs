use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(Slash)]
pub fn derive_slash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Slash for #name {
            // fn is_slash(&self) -> bool {
            //     true
            // }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Legacy)]
pub fn derive_legacy(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Legacy for #name {
            // fn is_legacy(&self) -> bool {
            //     true
            // }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Autocomplete)]
pub fn derive_autocomplete(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Autocomplete for #name {
            // fn is_autocomplete(&self) -> bool {
            //     true
            // }
        }
    };

    TokenStream::from(expanded)
}
