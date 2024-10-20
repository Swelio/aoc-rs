use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

use log::debug;
use pest::Parser;

use utils::SantaError;

use crate::day_05_parsers::{Stack, StackId};

#[derive(pest_derive::Parser)]
#[grammar = "assets/day_05_procedure.pest"]
struct ProcedureParser;

pub fn run_procedure<F, I>(
    stacks: &mut HashMap<StackId, Stack>,
    input: I,
    moving_function: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(u8, &mut Stack, &mut Stack) -> Result<(), Box<dyn Error>>,
    I: BufRead,
{
    debug!("Run procedure.");

    for line in input.lines().flat_map(|x| x.ok()) {
        debug!("Processing line: {}", line);

        let mut line_pairs = ProcedureParser::parse(Rule::move_line, &line)?
            .next()
            .unwrap()
            .into_inner();

        let moved_count: u8 = line_pairs.next().unwrap().as_str().parse()?;
        let src_stack_id: StackId = line_pairs.next().unwrap().as_str().parse()?;
        let dst_stack_id: StackId = line_pairs.next().unwrap().as_str().parse()?;

        debug!(
            "Moving {} crates from stack {} to stack {}",
            moved_count, src_stack_id, dst_stack_id
        );

        let mut src_stack = stacks.remove(&src_stack_id).ok_or_else(|| {
            SantaError::WrongProcedure(format!(
                "Unexisting source stack '{}' for procedure.",
                src_stack_id
            ))
        })?;
        let mut dst_stack = stacks.remove(&dst_stack_id).ok_or_else(|| {
            SantaError::WrongProcedure(format!(
                "Unexisting destination stack '{}' for procedure.",
                dst_stack_id
            ))
        })?;

        // Catch move result
        let move_result = moving_function(moved_count, &mut src_stack, &mut dst_stack);

        // Insert back both stacks
        stacks.insert(src_stack_id, src_stack);
        stacks.insert(dst_stack_id, dst_stack);

        // Finally check move result in case of error
        move_result?;
    }

    Ok(())
}
