use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn generate_units(range: impl Iterator<Item = usize>, prefix: &str) -> TokenStream {
    let units = range.map(|number| {
        let name = format_ident!("{prefix}{number:02}");
        quote!(pub struct #name;)
    });

    quote!(#(#units)*)
}
