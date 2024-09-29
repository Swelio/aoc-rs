use std::collections::{hash_map::Entry, HashMap, HashSet};

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::units::{day_ident, part_ident, year_ident};

pub fn expand_derive_executor(input: TokenStream) -> TokenStream {
    let exec_options =
        ExecutorOptions::from_derive_input(&parse_macro_input!(input as DeriveInput)).unwrap();
    let expected_days = exec_options.build_expected_days();
    let year_matching = expected_days.into_iter().flat_map(|(year, expected_days)| {
        (1..=31)
            .flat_map(move |day_number| {
                (1..=2u8).map(move |part_number| (year.0, day_number, part_number))
            })
            .map(
                move |(year, day, part)| match expected_days.get(&DayIdx(day)) {
                    None => quote! {
                        (#year, #day, #part) => unimplemented!(),
                    },
                    Some(_) => {
                        let year_name = year_ident(year as usize);
                        let day_name = day_ident(day as usize);
                        let part_name = part_ident(part as usize);

                        quote! {
                            (#year, #day, #part) => {
                                let solution = <Self as ::aoc_traits::solver::Solver<#year_name, #day_name, #part_name>>::solve(self, request.input());
                                ::aoc_traits::executor::solution::ChallengeSolution::new(&request, solution)
                            },
                        }
                    }
                },
            )
    });
    let executor_name = exec_options.ident;

    quote! {
        impl ::aoc_traits::executor::Executor for #executor_name {
            fn resolve(&self, request: ::aoc_traits::executor::request::ChallengeRequest) -> ::aoc_traits::executor::solution::ChallengeSolution {
                match request.id().as_tuple() {
                    #(#year_matching)*
                    _ => unimplemented!()
                }
            }
        }
    }
    .into()
}

#[derive(Debug, darling::FromDeriveInput)]
#[darling(attributes(executor))]
struct ExecutorOptions {
    ident: syn::Ident,
    #[darling(multiple, rename = "year")]
    years: Vec<Year>,
}

impl ExecutorOptions {
    fn build_expected_days(&self) -> HashMap<YearIdx, HashSet<DayIdx>> {
        self.years.iter().fold(HashMap::new(), |mut acc, year| {
            match acc.entry(year.year) {
                Entry::Vacant(vacant_entry) => {
                    let days = (year.first_day.0..=year.last_day.0).map(DayIdx).collect();
                    vacant_entry.insert(days);
                }
                Entry::Occupied(mut occupied_entry) => {
                    let new_days = (year.first_day.0..=year.last_day.0).map(DayIdx);
                    let known_days = occupied_entry.get_mut();
                    known_days.extend(new_days);
                }
            }

            acc
        })
    }
}

#[derive(Debug, darling::FromMeta)]
struct Year {
    year: YearIdx,
    first_day: DayIdx,
    last_day: DayIdx,
}

#[derive(Clone, Copy, Debug, darling::FromMeta, PartialEq, Eq, Hash)]
struct YearIdx(u16);
#[derive(Clone, Copy, Debug, darling::FromMeta, PartialEq, Eq, Hash)]
struct DayIdx(u8);
