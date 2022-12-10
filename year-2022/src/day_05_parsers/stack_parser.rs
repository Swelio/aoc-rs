use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

use log::debug;
use pest::Parser;

use crate::day_05_parsers::{Stack, StackId};

#[derive(pest_derive::Parser)]
#[grammar = "assets/day_05_stacks.pest"]
struct StackParser;

pub fn parse_stacks<I>(input: I) -> Result<HashMap<StackId, Stack>, Box<dyn Error>>
where
    I: BufRead,
{
    let mut stacks: HashMap<StackId, Stack> = HashMap::new();
    let mut stacks_input = String::new();
    let mut stack_crates = HashMap::new();

    for line in input.lines().flat_map(|x| x.ok()) {
        // Blank line separating stacks and moves
        if line.trim().is_empty() {
            break;
        }

        stacks_input.push_str(&line);
        stacks_input.push_str("\n");
    }

    let stack_pairs = StackParser::parse(Rule::stack, &stacks_input)?
        .next()
        .unwrap();

    for pair in stack_pairs.into_inner() {
        match pair.as_rule() {
            Rule::stack_id_line => {
                for (stack_index, stack_id_pair) in pair.into_inner().enumerate() {
                    if let Some(stack_crates) = stack_crates.remove(&stack_index) {
                        debug!("Parsing stack id: '{}'", stack_id_pair.as_str());
                        let stack_id: StackId = stack_id_pair.as_str().trim().parse()?;
                        stacks.insert(stack_id, stack_crates);
                    }
                }
            }
            Rule::stack_crate_line => {
                for (stack_index, stack_crate) in pair.into_inner().enumerate() {
                    // Ensure the stack is registered
                    let entry = stack_crates.entry(stack_index).or_default();

                    debug!("Parse stack_crate rule: {:?}", stack_crate.as_rule());

                    match stack_crate.as_rule() {
                        Rule::stack_crate_id => {
                            entry.push(stack_crate.as_str().chars().nth(0).unwrap())
                        }
                        Rule::stack_empty_crate => continue,
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    for stack in stacks.values_mut() {
        stack.reverse();
    }

    Ok(stacks)
}
