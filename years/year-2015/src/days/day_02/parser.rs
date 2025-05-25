use aoc_core::{ParsingError, ParsingResult, components::TextInput, ports};
use winnow::{
    Parser,
    ascii::{digit1, multispace0, newline},
    combinator::{separated, seq, terminated},
};

use super::{components::Cuboid, input::Input};

impl ports::Parser<&TextInput> for Input {
    fn try_parse(input: &TextInput) -> ParsingResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    terminated(separated(1.., parse_cuboid, newline), multispace0)
        .map(Input::new)
        .parse_next(input)
}

fn parse_cuboid(input: &mut &str) -> winnow::Result<Cuboid> {
    let parse_dimension = || digit1.parse_to();
    seq!(parse_dimension(), _: 'x', parse_dimension(), _: 'x', parse_dimension())
        .map(|(length, width, height)| Cuboid::new(length, width, height))
        .parse_next(input)
}
