use aoc_core::{SantaError, SantaResult, components::TextInput, ports};
use winnow::{
    Parser,
    combinator::{dispatch, empty, fail, repeat},
    token::any,
};

use super::{components::Direction, input::Day01Input};

impl ports::Parser<TextInput> for Day01Input {
    fn try_parse(input: TextInput) -> SantaResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| SantaError::ParsingError(anyhow::format_err!("{err}")))
    }
}

pub fn parse_input(input: &mut &str) -> winnow::Result<Day01Input> {
    let parse_direction = dispatch! {any;
        '(' => empty.value(Direction::Up),
        ')' => empty.value(Direction::Down),
        _ => fail::<_, Direction, _>,
    };

    repeat(1.., parse_direction)
        .map(Day01Input::new)
        .parse_next(input)
}
