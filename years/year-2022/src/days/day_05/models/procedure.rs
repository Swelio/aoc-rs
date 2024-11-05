use std::str::FromStr;

use aoc_traits::error::AocError;
use winnow::{ascii::line_ending, seq, PResult, Parser};

use super::{
    cargo::{parse_cargo, Cargo},
    instruction::{parse_instructions, Instructions},
};

pub struct Procedure {
    pub cargo: Cargo,
    pub instructions: Instructions,
}

impl FromStr for Procedure {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_procedure.parse(input)?)
    }
}

fn parse_procedure(input: &mut &str) -> PResult<Procedure> {
    let procedure = seq! {Procedure {cargo: parse_cargo, _: (line_ending, line_ending), instructions: parse_instructions}}.parse_next(input)?;

    Ok(procedure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = include_str!("../input.txt");
        let _procedure = Procedure::from_str(input).unwrap();
    }
}
