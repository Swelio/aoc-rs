use std::collections::{hash_map::Entry, HashMap, HashSet};

pub type YearIdx = u16;
pub type DayIdx = u8;
pub type PartIdx = u8;

#[derive(Debug, darling::FromDeriveInput)]
#[darling(attributes(solve))]
pub struct DynamicSolverOptions {
    pub(super) ident: syn::Ident,
    #[darling(multiple, rename = "year")]
    years: Vec<Year>,
}

impl DynamicSolverOptions {
    pub fn build_year_days(&self) -> HashMap<YearIdx, HashSet<DayIdx>> {
        self.years.iter().fold(HashMap::new(), |mut acc, year| {
            match acc.entry(year.year) {
                Entry::Vacant(vacant_entry) => {
                    let days = (year.first_day..=year.last_day).collect();
                    vacant_entry.insert(days);
                }
                Entry::Occupied(mut occupied_entry) => {
                    let new_days = year.first_day..=year.last_day;
                    let known_days = occupied_entry.get_mut();
                    known_days.extend(new_days);
                }
            }

            acc
        })
    }
}

#[derive(Debug, darling::FromMeta)]
pub struct Year {
    year: YearIdx,
    first_day: DayIdx,
    last_day: PartIdx,
}
