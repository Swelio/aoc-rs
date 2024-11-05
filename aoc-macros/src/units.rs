use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_units<F: Fn(usize) -> syn::Ident>(
    items: impl Iterator<Item = usize>,
    ident_generator: F,
) -> TokenStream {
    let units = items.map(ident_generator);

    quote!(#(pub struct #units;)*)
}

pub fn year_ident(number: usize) -> syn::Ident {
    format_ident!("Year{number}")
}

pub fn day_ident(number: usize) -> syn::Ident {
    format_ident!("Day{number:02}")
}

pub fn part_ident(number: usize) -> syn::Ident {
    format_ident!("Part{number:02}")
}
