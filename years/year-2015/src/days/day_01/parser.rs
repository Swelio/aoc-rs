use aoc_core::{ParsingError, ParsingResult, components::TextInput, ports};
use winnow::{
    Parser,
    combinator::{dispatch, empty, fail, repeat},
    token::any,
};

use super::{components::Direction, input::Input};

impl ports::Parser<&TextInput> for Input {
    fn try_parse(input: &TextInput) -> ParsingResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    let parse_direction = dispatch! {any;
        '(' => empty.value(Direction::Up),
        ')' => empty.value(Direction::Down),
        _ => fail::<_, Direction, _>,
    };

    repeat(1.., parse_direction)
        .map(Input::new)
        .parse_next(input)
}
