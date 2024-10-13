//! Instructions are there: https://adventofcode.com/2022/day/1

mod input;
mod part_01;
mod part_02;
mod tests;

// use std::cmp::Reverse;
// use std::error::Error;
// use std::io::{BufRead, BufReader, Read, Seek};

// pub struct DailySolution;

// impl CodeSolution for DailySolution {
//     fn run<I>(input: I) -> Result<(), Box<dyn Error>>
//     where
//         I: Read + Seek,
//     {
//         let calories = collect_calories(BufReader::new(input))?;
//         let max_calories = get_top_calories(&calories, 1)?;
//         let podium_calories = get_top_calories(&calories, 3)?;

//         println!("The most calories carried are {}", max_calories);
//         println!("The top 3 Elves carries {} calories", podium_calories);

//         Ok(())
//     }
// }

// fn collect_calories<I>(input: I) -> Result<Vec<u32>, Box<dyn Error>>
// where
//     I: BufRead,
// {
//     // Count each Elf total calories
//     let mut calories = Vec::new();

//     for line in input.lines().flat_map(|x| x.ok()) {
//         let line = line.trim();

//         if line.is_empty() {
//             // Blank line
//             calories.push(0);
//         } else {
//             // Sum up the current value
//             let added: u32 = line.parse()?;
//             **calories.last_mut().get_or_insert(&mut 0) += added;
//         }
//     }

//     // Sort elves total calories
//     calories.sort_by_key(|x| Reverse(*x));

//     Ok(calories)
// }

// fn get_top_calories(calories: &[u32], top_number: usize) -> Result<u32, Box<dyn Error>> {
//     if calories.len() < top_number {
//         return Err(SantaError::InvalidInput("Input is incomplete or empty.".to_string()).into());
//     }

//     let top = &calories[..top_number];

//     Ok(top.iter().sum())
// }
