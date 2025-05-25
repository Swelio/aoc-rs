use proc_macro::TokenStream;
use syn::parse_macro_input;

mod combine_parsers;

#[proc_macro]
pub fn combine_parsers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as combine_parsers::MacroInput);
    let output = combine_parsers::expand(input);

    proc_macro::TokenStream::from(output)
}
