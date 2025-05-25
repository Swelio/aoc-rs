use aoc_core::{ParsingError, ParsingResult, components::TextInput, ports};
use winnow::{
    Parser,
    combinator::{dispatch, empty, fail, repeat},
    token::any,
};

use super::{Input, components::NextMove};

impl ports::Parser<&TextInput> for Input {
    fn try_parse(input: &TextInput) -> ParsingResult<Self> {
        parse_input
            .parse(input.as_ref())
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    repeat(1.., parse_move).map(Input::new).parse_next(input)
}

fn parse_move(input: &mut &str) -> winnow::Result<NextMove> {
    dispatch! {any;
        '^' => empty.value(NextMove::North),
        'v' => empty.value(NextMove::South),
        '>' => empty.value(NextMove::East),
        '<' => empty.value(NextMove::West),
        _ => fail::<_, NextMove, _>
    }
    .parse_next(input)
}
