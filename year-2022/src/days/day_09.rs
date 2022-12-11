//! Instructions are there: https://adventofcode.com/2022/day/9

use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use utils::{CodeSolution, SantaError};

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let mut buf_input = BufReader::new(input);
        let visited_positions = part_1(&mut buf_input)?;
        println!("Total visited positions: {}", visited_positions);

        buf_input.seek(SeekFrom::Start(0))?;

        let visited_positions = part_2(buf_input)?;
        println!(
            "Long rope tail total visited positions: {}",
            visited_positions
        );

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<usize, Box<dyn Error>>
where
    I: BufRead,
{
    let mut head = KnotPosition::new(0, 0);
    let mut tail = KnotPosition::new(0, 0);
    let mut visited_positions: HashSet<(isize, isize)> = [(tail.x, tail.y)].into_iter().collect();

    for line in input.lines() {
        let line = line?;
        let (raw_direction, raw_steps) = line
            .split_once(' ')
            .ok_or_else(|| SantaError::InvalidInput(format!("Invalid line: '{}'", line)))?;
        let head_direction = Direction::try_from(raw_direction)?;
        let steps = raw_steps.parse()?;

        for _ in 0..steps {
            head.update(head_direction);
            tail.follow(&head);
            visited_positions.insert((tail.x, tail.y));
        }
    }

    Ok(visited_positions.len())
}

fn part_2<I>(input: I) -> Result<usize, Box<dyn Error>>
where
    I: BufRead,
{
    let mut head = KnotPosition::new(0, 0);
    let mut rope = Vec::from([KnotPosition::new(0, 0); 8]);
    let tail = KnotPosition::new(0, 0);
    let mut visited_positions: HashSet<(isize, isize)> = [(tail.x, tail.y)].into_iter().collect();

    rope.push(tail);

    for line in input.lines() {
        let line = line?;
        let (raw_direction, raw_steps) = line
            .split_once(' ')
            .ok_or_else(|| SantaError::InvalidInput(format!("Invalid line: '{}'", line)))?;
        let head_direction = Direction::try_from(raw_direction)?;
        let steps = raw_steps.parse()?;

        for _ in 0..steps {
            head.update(head_direction);

            let mut previous_knot = head;

            for knot in rope.iter_mut() {
                knot.follow(&previous_knot);
                previous_knot = *knot;
            }

            let tail = rope.last().unwrap();

            visited_positions.insert((tail.x, tail.y));
        }
    }

    Ok(visited_positions.len())
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = SantaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = match value {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            _ => {
                return Err(SantaError::InvalidInput(format!(
                    "Unknown direction '{}'",
                    value
                )))
            }
        };

        Ok(direction)
    }
}

#[derive(Copy, Clone)]
struct KnotPosition {
    x: isize,
    y: isize,
}

impl KnotPosition {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn update(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.y -= 1,
            Direction::Up => self.x += 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x -= 1,
        }
    }

    fn follow(&mut self, head: &Self) {
        // Powered distance can be up to 2 because it is only possible when head is on diagonal of tail
        if self.get_powered_distance(head) <= 2 {
            return;
        }

        if head.x > self.x {
            self.x += 1;
        } else if head.x < self.x {
            self.x -= 1;
        }

        if head.y > self.y {
            self.y += 1;
        } else if head.y < self.y {
            self.y -= 1;
        }
    }

    /// Non-squared euclidean distance
    fn get_powered_distance(&self, head: &KnotPosition) -> usize {
        head.x.abs_diff(self.x).pow(2) + head.y.abs_diff(self.y).pow(2)
    }
}

#[cfg(test)]
mod test_day {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_part_1() {
        const SAMPLE_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let visited_positions = part_1(input_cursor).unwrap();

        assert_eq!(visited_positions, 13);
    }

    #[test]
    fn test_part_2() {
        const SAMPLE_INPUT: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let visited_positions = part_2(input_cursor).unwrap();

        assert_eq!(visited_positions, 36);
    }
}
