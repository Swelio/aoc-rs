use aoc_core::{SantaError, components::TextInput, ports};
use winnow::{Parser, combinator::alt};

use crate::days::{DayInput, day_01};

#[derive(Debug)]
pub struct Year2015Input(DayInput);

impl ports::Parser<TextInput> for Year2015Input {
    fn try_parse(input: TextInput) -> aoc_core::SantaResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| SantaError::ParsingError(anyhow::format_err!("{err}")))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<Year2015Input> {
    alt((day_01::parse_input.map(DayInput::Day01),))
        .map(Year2015Input)
        .parse_next(input)
}
