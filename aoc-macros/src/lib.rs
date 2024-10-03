mod derive_dynamic_solver;
mod units;

use derive_dynamic_solver::expand_derive_dynamic_solver;
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

#[proc_macro_derive(DynamicSolver, attributes(solve))]
pub fn derive_dynamic_solver(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_derive_dynamic_solver(input)
}
