//! Instructions are there: https://adventofcode.com/2022/day/2

use std::cmp::Ordering;
use std::error::Error;
use std::io::{BufRead, BufReader, Cursor, Read};

use utils::{CodeSolution, SantaError};

type Score = u16;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(mut input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read,
    {
        let mut full_input = String::new();
        input
            .read_to_string(&mut full_input)
            .expect("Invalid input.");
        let strategy_score = part_1(BufReader::new(Cursor::new(full_input.clone())))?;
        println!("Total score of the strategy is: {}", strategy_score);

        let full_score = part_2(BufReader::new(Cursor::new(full_input)))?;
        println!("Full score of the strategy is: {}", full_score);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<Score, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_score = 0;

    for line in input.lines().flat_map(|x| x.ok()) {
        let line = line.trim();
        let (opponent_move, self_move) = match line.split_once(' ') {
            Some(choices) => (Moves::try_from(choices.0)?, Moves::try_from(choices.1)?),
            None => {
                return Err(
                    SantaError::InvalidInput("Player choices not found.".to_string()).into(),
                )
            }
        };

        total_score += self_move.get_score();

        match self_move.partial_cmp(&opponent_move) {
            None => unreachable!(),
            Some(Ordering::Equal) => total_score += 3,
            Some(Ordering::Greater) => total_score += 6,
            _ => {}
        }
    }

    Ok(total_score)
}

fn part_2<I>(input: I) -> Result<Score, Box<dyn Error>>
where
    I: BufRead,
{
    let mut total_score = 0;

    for line in input.lines().flat_map(|x| x.ok()) {
        let line = line.trim();
        let (opponent_move, self_advice) = match line.split_once(' ') {
            Some(choices) => (Moves::try_from(choices.0)?, choices.1),
            None => {
                return Err(
                    SantaError::InvalidInput("Player choices not found.".to_string()).into(),
                );
            }
        };

        let self_move = match self_advice {
            "X" => match opponent_move {
                Moves::Rock => Moves::Scissors,
                Moves::Paper => Moves::Rock,
                Moves::Scissors => Moves::Paper,
            },
            "Y" => {
                total_score += 3;
                opponent_move
            }
            "Z" => {
                total_score += 6;
                match opponent_move {
                    Moves::Rock => Moves::Paper,
                    Moves::Paper => Moves::Scissors,
                    Moves::Scissors => Moves::Rock,
                }
            }
            _ => {
                return Err(SantaError::InvalidInput(format!(
                    "Unknown move for '{}'",
                    self_advice
                ))
                .into());
            }
        };

        total_score += self_move.get_score();
    }

    Ok(total_score)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn get_score(self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for Moves {
    type Error = SantaError;

    fn try_from(letter: &str) -> Result<Self, Self::Error> {
        match letter {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(SantaError::InvalidInput(format!(
                "Unknown letter '{}'",
                letter
            ))),
        }
    }
}

impl PartialOrd for Moves {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        })
    }
}

#[cfg(test)]
mod test_day {
    use std::io::Cursor;

    use super::*;

    const SAMPLE_INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_part_1() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let strategy_score = part_1(BufReader::new(input_cursor)).unwrap();
        assert_eq!(strategy_score, 15);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let strategy_score = part_2(BufReader::new(input_cursor)).unwrap();
        assert_eq!(strategy_score, 12);
    }
}
