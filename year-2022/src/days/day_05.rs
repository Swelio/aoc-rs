//! Instructions are there: https://adventofcode.com/2022/day/5

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use log::debug;

use utils::{CodeSolution, SantaError};

use crate::day_05_parsers::{
    procedure_parser::run_procedure, stack_parser::parse_stacks, Stack, StackId,
};

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let mut buf_input = BufReader::new(input);
        let stacks = parse_stacks(&mut buf_input)?;
        let start_seek = buf_input.stream_position()?;

        let top_crates = part_1(stacks.clone(), &mut buf_input)?;
        println!("Top crates for CrateMove 9000 are: {}", top_crates);

        // Reset input cursor just after the stacks initialization
        buf_input.seek(SeekFrom::Start(start_seek))?;

        let top_crates = part_2(stacks, &mut buf_input)?;
        println!("Top crates for CrateMove 9001 are: {}", top_crates);

        Ok(())
    }
}

fn part_1<I>(mut stacks: HashMap<StackId, Stack>, input: I) -> Result<String, Box<dyn Error>>
where
    I: BufRead,
{
    run_procedure(&mut stacks, input, move_crate_old_crane)?;
    Ok(collect_top_crates(&stacks))
}

fn part_2<I>(mut stacks: HashMap<StackId, Stack>, input: I) -> Result<String, Box<dyn Error>>
where
    I: BufRead,
{
    run_procedure(&mut stacks, input, move_crate_new_crane)?;
    Ok(collect_top_crates(&stacks))
}

fn collect_top_crates(stacks: &HashMap<StackId, Stack>) -> String {
    let mut stack_ids: Vec<StackId> = stacks.keys().copied().collect();
    stack_ids.sort();

    debug!("Processing stacks: {:?}", stack_ids);

    let mut message = String::with_capacity(stack_ids.len());

    for stack_id in stack_ids {
        debug!(
            "Stack for {}: {:?}",
            stack_id,
            stacks.get(&stack_id).unwrap()
        );

        if let Some(&stack_crate) = stacks.get(&stack_id).unwrap().last() {
            message.push(stack_crate);
        }
    }

    message
}

/// Moving function for part 1
fn move_crate_old_crane(
    moved_count: u8,
    src_stack: &mut Stack,
    dst_stack: &mut Stack,
) -> Result<(), Box<dyn Error>> {
    debug!(
        "Move {} crates from stack {:?} to stack {:?}",
        moved_count, src_stack, dst_stack
    );

    for _ in 0..moved_count {
        dst_stack.push(
            src_stack
                .pop()
                .ok_or_else(|| SantaError::WrongProcedure("Source stack is empty.".to_string()))?,
        );
    }

    Ok(())
}

/// Moving function for part 2
fn move_crate_new_crane(
    moved_count: u8,
    src_stack: &mut Stack,
    dst_stack: &mut Stack,
) -> Result<(), Box<dyn Error>> {
    debug!(
        "Move {} crates from stack {:?} to stack {:?}",
        moved_count, src_stack, dst_stack
    );

    let moved_crates = src_stack.drain((src_stack.len() - moved_count as usize)..);
    dst_stack.extend(moved_crates);

    Ok(())
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};
    use std::sync::Once;

    use super::*;

    static LOG_INIT: Once = Once::new();

    const SAMPLE_INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    fn setup() {
        LOG_INIT.call_once(env_logger::init);
    }

    #[test]
    fn test_stack_parsing() {
        setup();

        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let stacks = parse_stacks(BufReader::new(input_cursor)).unwrap();

        assert!(stacks.contains_key(&1));
        assert!(stacks.contains_key(&2));
        assert!(stacks.contains_key(&3));

        assert_eq!(stacks.get(&1).unwrap(), &['Z', 'N']);
        assert_eq!(stacks.get(&2).unwrap(), &['M', 'C', 'D']);
        assert_eq!(stacks.get(&3).unwrap(), &['P']);
    }

    #[test]
    fn test_part_1() {
        setup();

        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let mut buf_input = BufReader::new(input_cursor);
        let stacks = parse_stacks(&mut buf_input).unwrap();
        let message = part_1(stacks, buf_input).unwrap();
        assert_eq!(message, "CMZ");
    }

    #[test]
    fn test_part_2() {
        setup();

        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let mut buf_input = BufReader::new(input_cursor);
        let stacks = parse_stacks(&mut buf_input).unwrap();
        let message = part_2(stacks, buf_input).unwrap();
        assert_eq!(message, "MCD");
    }
}
