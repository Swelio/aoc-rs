use aoc_core::{ParsingError, components::TextInput, ports};
use winnow::{
    Parser,
    ascii::multispace0,
    combinator::{repeat, terminated},
    stream::AsChar,
    token::any,
};

use super::Input;

impl ports::Parser<&TextInput> for Input {
    fn try_parse(input: &TextInput) -> aoc_core::ParsingResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    terminated(
        repeat(6..=8, any.verify(|c: &char| c.is_alpha())),
        multispace0,
    )
    .parse_next(input)
    .map(Input::new)
}
