use aoc_core::{
    SantaError, SantaResult,
    archetypes::DaySolution,
    components::{Solution, TextInput, parts::Part1},
    ports,
};
use winnow::{Parser, combinator::alt};

use crate::days::{DayInput, day_01};

#[derive(Debug)]
pub struct YearInput(DayInput);

impl ports::Parser<TextInput> for YearInput {
    fn try_parse(input: TextInput) -> SantaResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| SantaError::ParsingError(anyhow::format_err!("{err}")))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<YearInput> {
    alt((day_01::parse_input.map(DayInput::Day01),))
        .map(YearInput)
        .parse_next(input)
}

impl ports::Challenge<DaySolution<Solution<Part1>>> for YearInput {
    fn solve(&self) -> SantaResult<DaySolution<Solution<Part1>>> {
        let wrap_solution = |day_number, solutions| {
            DaySolution::<Solution<Part1>>::new(2015, day_number, solutions)
        };

        match &self.0 {
            DayInput::Day01(day01_input) => day01_input
                .solve()
                .map(|solutions| wrap_solution(1, solutions)),
        }
    }
}
