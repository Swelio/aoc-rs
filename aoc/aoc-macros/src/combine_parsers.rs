use proc_macro2::TokenStream;
use quote::quote;
use syn::{Token, TypePath, parse::ParseStream, punctuated::Punctuated};

pub fn expand(input: MacroInput) -> TokenStream {
    let parse_return = input.parse_return;
    let parsers = input
        .parsers
        .iter()
        .map(|parser| quote! (#parser::try_parse(input).map(|solver| -> #parse_return { Box::new(solver) })));
    let assembly = parsers
        .reduce(|acc, parser| quote! (#acc.or_else(|_| #parser)))
        .expect("input must have at least one parser");
    let terminate = quote! (#assembly.map_err(|_| ParsingError::from("no valid parser found")));

    quote!(|input| #terminate)
}

#[derive(Debug, Clone)]
pub struct MacroInput {
    parse_return: TypePath,
    parsers: Vec<TypePath>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parse_return = input.parse()?;
        let _: Token![,] = input.parse()?;
        let parsers = Punctuated::<TypePath, Token![,]>::parse_separated_nonempty(input)?;

        Ok(Self {
            parse_return,
            parsers: parsers.into_iter().collect(),
        })
    }
}
