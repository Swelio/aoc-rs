//! Instructions are there: https://adventofcode.com/2022/day/3

use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use utils::{CodeSolution, SantaError};

type Priority = u16;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(mut input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let priorities_sum = part_1(BufReader::new(&mut input))?;
        println!("Total sum of priorities is: {}", priorities_sum);

        input.seek(SeekFrom::Start(0))?;

        let group_priorities = part_2(BufReader::new(input))?;
        println!("Group priorities sum is: {}", group_priorities);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<Priority, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_priorities = 0;

    for line in input.lines().flat_map(|x| x.ok()) {
        let line = line.trim();

        if line.is_empty() {
            return Err(SantaError::InvalidInput("Empty line into input.".to_string()).into());
        }

        let common_item = match get_common_item(line) {
            Some(item) => item,
            None => {
                return Err(SantaError::InvalidInput(format!(
                    "No common input in line: '{}'",
                    line
                ))
                .into());
            }
        };
        let priority = get_item_priority(common_item)?;
        total_priorities += priority;
    }

    Ok(total_priorities)
}

fn part_2<I>(input: I) -> Result<Priority, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_priorities = 0;
    let mut lines_iterator = input.lines().flat_map(|x| x.ok());

    while let Some(new_line) = lines_iterator.next() {
        let mut group_set: HashSet<char> = new_line.chars().collect();

        for _ in 0..2 {
            let new_line: HashSet<char> = lines_iterator
                .next()
                .ok_or_else(|| SantaError::InvalidInput("Missing lines in input.".to_string()))?
                .chars()
                .collect();
            group_set.retain(|item| new_line.contains(item));
        }

        if group_set.len() != 1 {
            return Err(SantaError::InvalidInput(
                "Group chunk does not have exactly one common item.".to_string(),
            )
            .into());
        }

        let priority = get_item_priority(group_set.drain().next().unwrap())?;

        total_priorities += priority;
    }

    Ok(total_priorities)
}

fn get_common_item(line: &str) -> Option<char> {
    let midline_index = line.len() / 2;
    let (first_part, second_part) = (
        line[..midline_index].chars().collect::<HashSet<char>>(),
        line[midline_index..].chars().collect::<HashSet<char>>(),
    );

    first_part.intersection(&second_part).copied().next()
}

fn get_item_priority(item: char) -> Result<Priority, SantaError> {
    if ('a'..='z').contains(&item) {
        Ok(item as Priority - b'a' as Priority + 1)
    } else if ('A'..='Z').contains(&item) {
        Ok(item as Priority - b'A' as Priority + 27)
    } else {
        Err(SantaError::InvalidInput(format!(
            "Unknown item type '{}'",
            item
        )))
    }
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part_1() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let priority_sum = part_1(BufReader::new(input_cursor)).unwrap();
        assert_eq!(priority_sum, 157);
    }
    #[test]
    fn test_part_2() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let priority_sum = part_2(BufReader::new(input_cursor)).unwrap();
        assert_eq!(priority_sum, 70);
    }
}
