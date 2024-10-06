use std::collections::{HashMap, HashSet};

use quote::quote;

use crate::units::{day_ident, part_ident, year_ident};

use super::options::{DayIdx, PartIdx, YearIdx};

const MIN_DAY: DayIdx = 1;
const MAX_DAY: DayIdx = 31;

const MIN_PART: PartIdx = 1;
const MAX_PART: PartIdx = 2;

pub fn expand_years(
    year_days: HashMap<YearIdx, HashSet<DayIdx>>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let day_range = || {
        (MIN_DAY..=MAX_DAY).flat_map(move |day| (MIN_PART..=MAX_PART).map(move |part| (day, part)))
    };

    year_days
        .into_iter()
        .flat_map(move |(year, expected_days)| {
            day_range().map(move |(day, part)| expand_day(year, day, part, &expected_days))
        })
}

fn expand_day(
    year: YearIdx,
    day: DayIdx,
    part: PartIdx,
    expected_days: &HashSet<DayIdx>,
) -> proc_macro2::TokenStream {
    expected_days.contains(&day).then(|| {
        let year_name = year_ident(year as usize);
        let day_name = day_ident(day as usize);
        let part_name = part_ident(part as usize);

        quote! {
            (#year, #day, #part) => {
                let solution = <Self as ::aoc_traits::solver::Solver<#year_name, #day_name, #part_name>>::solve(self, request.input())?;
                Ok(::aoc_traits::dynamic_solver::ChallengeSolution::new(&request, solution))
            },
        }
    }).unwrap_or_else(|| quote! {
        (#year, #day, #part) => unimplemented!(),
    })
}
