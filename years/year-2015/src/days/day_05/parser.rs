use aoc_core::{ParsingError, components::TextInput, ports};
use winnow::{
    Parser,
    ascii::{multispace0, newline},
    combinator::{separated, terminated},
    stream::AsChar,
    token::take_while,
};

use super::{Input, components::Line};

impl ports::Parser<&TextInput> for Input {
    fn try_parse(input: &TextInput) -> aoc_core::ParsingResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    terminated(
        separated(
            1..,
            take_while(1..=16, AsChar::is_alpha).map(|line: &str| Line::new(line.to_string())),
            newline,
        ),
        multispace0,
    )
    .map(Input::new)
    .parse_next(input)
}
