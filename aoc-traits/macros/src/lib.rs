mod derive_executor;
mod units;

use derive_executor::expand_derive_executor;
use units::{day_ident, generate_units, part_ident, year_ident};

#[proc_macro]
pub fn generate_year_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(2015..=2023, year_ident).into()
}

#[proc_macro]
pub fn generate_day_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(1..=31, day_ident).into()
}

#[proc_macro]
pub fn generate_part_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(1..=2, part_ident).into()
}

#[proc_macro_derive(Executor, attributes(executor))]
pub fn derive_executor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_derive_executor(input)
}
