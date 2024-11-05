use std::{cell::RefCell, str::FromStr};

use aoc_traits::prelude::*;

use crate::Year2022Solver;

use super::models::{cargo::Stack, instruction::Instruction, procedure::Procedure};

impl Solver<Year2022, Day05, Part01> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let Procedure {
            cargo,
            instructions,
        } = Procedure::from_str(input.as_ref())?;

        let cargo = cargo.into_iter().map(RefCell::new).collect::<Vec<_>>();

        instructions
            .into_inner()
            .into_iter()
            .try_for_each(|instruction| move_crates(&cargo, instruction))?;

        let flag = cargo
            .into_iter()
            .filter_map(|stack| stack.into_inner().last().copied())
            .map(|item| item.into_inner())
            .collect::<String>();

        Ok(Flag::from(flag))
    }
}

fn move_crates(cargo: &[RefCell<Stack>], instruction: Instruction) -> AocResult<()> {
    let source = cargo
        .get(instruction.from.into_inner())
        .ok_or(AocError::Input)?;
    let destination = cargo
        .get(instruction.to.into_inner())
        .ok_or(AocError::Input)?;
    let count = instruction.quantity.into_inner();

    for _ in 0..count {
        let mut source = source.borrow_mut();
        let mut destination = destination.borrow_mut();

        let current = source.pop().ok_or(AocError::Input)?;
        destination.push(current);
    }

    Ok(())
}
