mod units;

use units::generate_units;

#[proc_macro]
pub fn generate_year_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(2015..=2023, "Year").into()
}

#[proc_macro]
pub fn generate_day_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(1..=31, "Day").into()
}

#[proc_macro]
pub fn generate_part_units(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_units(1..=2, "Part").into()
}
