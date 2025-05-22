use aoc_core::{SantaError, SantaResult, components::TextInput, ports};
use winnow::{
    Parser,
    ascii::{digit1, newline},
    combinator::{separated, seq},
};

use super::{components::Cuboid, input::Input};

impl ports::Parser<TextInput> for Input {
    fn try_parse(input: TextInput) -> SantaResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| SantaError::ParsingError(anyhow::format_err!("{err}")))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    separated(1.., parse_cuboid, newline)
        .map(Input::new)
        .parse_next(input)
}

fn parse_cuboid(input: &mut &str) -> winnow::Result<Cuboid> {
    let parse_dimension = || digit1.parse_to();
    seq!(parse_dimension(), _: 'x', parse_dimension(), _: 'x', parse_dimension())
        .map(|(length, width, height)| Cuboid::new(length, width, height))
        .parse_next(input)
}
