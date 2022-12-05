//! Instructions are there: https://adventofcode.com/2022/day/1

use std::cmp::Reverse;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use utils::{CodeSolution, SantaError};

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read,
    {
        let calories = collect_calories(BufReader::new(input))?;
        let max_calories = get_top_calories(&calories, 1)?;
        let podium_calories = get_top_calories(&calories, 3)?;

        println!("The most calories carried are {}", max_calories);
        println!("The top 3 Elves carries {} calories", podium_calories);

        Ok(())
    }
}

fn collect_calories<I>(input: I) -> Result<Vec<u32>, Box<dyn Error>>
where
    I: BufRead,
{
    // Count each Elf total calories
    let mut calories = Vec::new();

    for line in input.lines().flat_map(|x| x.ok()) {
        let line = line.trim();

        if line.is_empty() {
            // Blank line
            calories.push(0);
        } else {
            // Sum up the current value
            let added: u32 = line.parse()?;
            **calories.last_mut().get_or_insert(&mut 0) += added;
        }
    }

    // Sort elves total calories
    calories.sort_by_key(|x| Reverse(*x));

    Ok(calories)
}

fn get_top_calories(calories: &[u32], top_number: usize) -> Result<u32, Box<dyn Error>> {
    if calories.len() < top_number {
        return Err(SantaError::InvalidInput("Input is incomplete or empty.".to_string()).into());
    }

    let top = &calories[..top_number];

    Ok(top.iter().sum())
}

#[cfg(test)]
mod test_day {
    use std::io::Cursor;

    use super::*;

    const SAMPLE_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part_1() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let calories = collect_calories(input_cursor).unwrap();
        let result = get_top_calories(&calories, 1).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let calories = collect_calories(input_cursor).unwrap();
        let result = get_top_calories(&calories, 3).unwrap();
        assert_eq!(result, 45000);
    }
}
