use std::str::FromStr;

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::separated,
    seq, PResult, Parser,
};

#[nutype(derive(Debug, AsRef))]
pub struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_instructions.parse(input)?)
    }
}

pub fn parse_instructions(input: &mut &str) -> PResult<Instructions> {
    let mut parser = separated(1.., parse_instruction, line_ending);
    let instructions = parser.parse_next(input)?;

    Ok(Instructions::new(instructions))
}

#[derive(Debug)]
pub struct Instruction {
    pub quantity: Quantity,
    pub from: StackIndex,
    pub to: StackIndex,
}

#[nutype(derive(Debug, Clone, Copy))]
pub struct Quantity(usize);

#[nutype(derive(Debug, Clone, Copy))]
pub struct StackIndex(usize);

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_instruction.parse(input)?)
    }
}

fn parse_instruction(input: &mut &str) -> PResult<Instruction> {
    let instruction = seq! {Instruction {
        _: "move ",
        quantity: digit1.parse_to().map(Quantity::new),
        _: " from ",
        from: digit1.parse_to().map(|index: usize| StackIndex::new(index - 1)),
        _: " to ",
        to: digit1.parse_to().map(|index: usize| StackIndex::new(index - 1))
    }}
    .parse_next(input)?;

    Ok(instruction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parsing() {
        let input = "move 3 from 1 to 3";
        let _ = Instruction::from_str(input).unwrap();
    }
}
