//! Instructions are there: https://adventofcode.com/2022/day/4

use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::ops::RangeInclusive;

use utils::{CodeSolution, SantaError};

type Pairs = u16;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(mut input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let contained_pairs = part_1(BufReader::new(&mut input))?;
        println!("Total contained pairs: {}", contained_pairs);

        input.seek(SeekFrom::Start(0))?;

        let overlapped_pairs = part_2(BufReader::new(input))?;
        println!("Total overlapped pairs: {}", overlapped_pairs);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<Pairs, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_contained_pairs = 0;

    for line in input.lines().flat_map(|x| x.ok()) {
        let (first_pair, second_pair) = line
            .split_once(',')
            .ok_or_else(|| SantaError::InvalidInput("Invalid input.".to_string()))?;
        let first_pair_range = get_pair_range(first_pair)?;
        let second_pair_range = get_pair_range(second_pair)?;

        if (first_pair_range.contains(second_pair_range.start())
            && first_pair_range.contains(second_pair_range.end()))
            || (second_pair_range.contains(first_pair_range.start())
                && second_pair_range.contains(first_pair_range.end()))
        {
            total_contained_pairs += 1;
        }
    }

    Ok(total_contained_pairs)
}

fn part_2<I>(input: I) -> Result<Pairs, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_overlapping_pairs = 0;

    for line in input.lines().flat_map(|x| x.ok()) {
        let (first_pair, second_pair) = line
            .split_once(',')
            .ok_or_else(|| SantaError::InvalidInput("Invalid input.".to_string()))?;
        let first_pair_range = get_pair_range(first_pair)?;
        let second_pair_range = get_pair_range(second_pair)?;

        if first_pair_range.contains(second_pair_range.start())
            || first_pair_range.contains(second_pair_range.end())
            || second_pair_range.contains(first_pair_range.start())
            || second_pair_range.contains(first_pair_range.end())
        {
            total_overlapping_pairs += 1;
        }
    }

    Ok(total_overlapping_pairs)
}

fn get_pair_range(pair: &str) -> Result<RangeInclusive<u16>, Box<dyn Error>> {
    let (bound_0, bound_1) = pair
        .split_once('-')
        .ok_or_else(|| SantaError::InvalidInput("Invalid input.".to_string()))?;

    Ok(bound_0.trim().parse::<u16>()?..=bound_1.trim().parse::<u16>()?)
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_1() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let contained_pairs = part_1(BufReader::new(input_cursor)).unwrap();
        assert_eq!(contained_pairs, 2);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let overlapping_pairs = part_2(BufReader::new(input_cursor)).unwrap();
        assert_eq!(overlapping_pairs, 4);
    }
}
