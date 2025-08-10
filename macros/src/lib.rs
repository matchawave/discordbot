use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(SlashWithAutocomplete)]
pub fn derive_slash_with_autocomplete(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl SlashWithAutocomplete for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(SlashWithLegacy)]
pub fn derive_slash_with_legacy(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl SlashWithLegacy for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(SlashWithLegacyAutocomplete)]
pub fn derive_slash_with_legacy_autocomplete(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl SlashWithLegacyAutocomplete for #name {}
    };

    TokenStream::from(expanded)
}
