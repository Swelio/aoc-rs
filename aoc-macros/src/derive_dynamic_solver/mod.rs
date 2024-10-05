mod expand_year;
mod options;

use darling::FromDeriveInput;
use expand_year::expand_years;
use options::DynamicSolverOptions;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn expand_derive_dynamic_solver(input: TokenStream) -> TokenStream {
    let exec_options =
        DynamicSolverOptions::from_derive_input(&parse_macro_input!(input as DeriveInput)).unwrap();
    let year_days = exec_options.build_year_days();
    let expanded_years = expand_years(year_days);
    let dynamic_solver_name = exec_options.ident;

    quote! {
        impl ::aoc_traits::dynamic_solver::DynamicSolver for #dynamic_solver_name {
            fn resolve(&self, request: ::aoc_traits::dynamic_solver::ChallengeRequest) -> ::aoc_traits::error::AocResult<::aoc_traits::dynamic_solver::ChallengeSolution> {
                let challenge_id = request.id();
                match challenge_id.as_tuple() {
                    #(#expanded_years)*
                    _ => return Err(::aoc_traits::error::AocError::NotImplemented(challenge_id)),
                }
            }
        }
    }
    .into()
}
