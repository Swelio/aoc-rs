//! Instructions are there: https://adventofcode.com/2022/day/14

use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use utils::{CodeSolution, SantaError};

type Axis = i32;
type Coordinates = (Axis, Axis);

type CaveShape = HashSet<Coordinates>;

const SAND_SOURCE: Coordinates = (500, 0);

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let buf_input = BufReader::new(input);
        let mut cave_shape = parse_cave_shape(buf_input)?;

        let mut total_sand = part_1(&mut cave_shape);
        println!("Total fallen sand: {}", total_sand);

        total_sand += part_2(cave_shape);
        println!("Total fallen sand on the cave floor: {}", total_sand);

        Ok(())
    }
}

fn part_1(cave_shape: &mut CaveShape) -> u32 {
    let mut total_sand = 0;
    let floor_depth = deepest_rock(cave_shape);

    while let Some(resting_sand) = SandUnit::new(cave_shape)
        .take_while(|(_, sand_depth)| *sand_depth <= floor_depth)
        .last()
        .filter(|(_, sand_depth)| *sand_depth < floor_depth)
    {
        cave_shape.insert(resting_sand);
        total_sand += 1;
    }

    total_sand
}

fn part_2(mut cave_shape: CaveShape) -> u32 {
    // Count the sand source as 1
    let mut total_sand = 1;
    let floor_depth = deepest_rock(&cave_shape) + 2;

    while let Some(resting_sand) = SandUnit::new(&cave_shape)
        .take_while(|(_, sand_depth)| *sand_depth < floor_depth)
        .last()
    {
        cave_shape.insert(resting_sand);
        total_sand += 1;
    }

    total_sand
}

fn parse_cave_shape<I>(input: I) -> Result<CaveShape, Box<dyn Error>>
where
    I: BufRead,
{
    let mut cave_shape = CaveShape::new();

    for line in input.lines() {
        let line_coordinates = match line_parser(&line?) {
            Ok((_, pairs)) => pairs,
            Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
                return Err(SantaError::InvalidInput(err.to_string()).into())
            }
            Err(_) => unreachable!(),
        };
        for pair in line_coordinates.windows(2) {
            register_rock(&mut cave_shape, pair[0], pair[1]);
        }
    }

    Ok(cave_shape)
}

fn line_parser(line: &str) -> IResult<&str, Vec<Coordinates>> {
    separated_list1(tag(" -> "), coordinates_parser)(line)
}

fn coordinates_parser(input: &str) -> IResult<&str, Coordinates> {
    separated_pair(axis_parser, tag(","), axis_parser)(input)
}

fn axis_parser(input: &str) -> IResult<&str, Axis> {
    map_res(digit1, |parsed_axis| Axis::from_str_radix(parsed_axis, 10))(input)
}

/// Register a rock line
fn register_rock(cave_shape: &mut CaveShape, start: Coordinates, end: Coordinates) {
    for x in start.0.min(end.0)..=start.0.max(end.0) {
        for y in start.1.min(end.1)..=start.1.max(end.1) {
            cave_shape.insert((x, y));
        }
    }
}

fn deepest_rock(cave_shape: &CaveShape) -> Axis {
    cave_shape
        .iter()
        .copied()
        .chain([(SAND_SOURCE.0, SAND_SOURCE.1 + 1)])
        .map(|(_, y)| y)
        .max()
        .unwrap()
}

/// A sand unit which is falling and can be iterated.
/// Each step, it updates to the new available position, until reaching a resting position, or the floor.
#[derive(Clone, Debug)]
struct SandUnit<'a> {
    cave_shape: &'a CaveShape,
    current_position: Coordinates,
}

impl<'a> SandUnit<'a> {
    fn new(cave_shape: &'a CaveShape) -> Self {
        Self {
            cave_shape,
            current_position: SAND_SOURCE,
        }
    }
}

impl<'a> Iterator for SandUnit<'a> {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        for angle in [
            std::f64::consts::FRAC_PI_2,       // straight down
            3.0 * std::f64::consts::FRAC_PI_4, // down-left
            std::f64::consts::FRAC_PI_4,       // down-right
        ] {
            let new_x = self.current_position.0 + (angle.cos().round() as Axis);
            let new_y = self.current_position.1 + (angle.sin().round() as Axis);

            let new_position = (new_x, new_y);

            // Only air found, continue the fall
            if !self.cave_shape.contains(&new_position) {
                self.current_position = new_position;
                return Some(new_position);
            }
        }

        None
    }
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_cave_parsing() {
        let input_buf = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let mut cave_shape = CaveShape::new();

        register_rock(&mut cave_shape, (498, 4), (498, 6));
        register_rock(&mut cave_shape, (498, 6), (496, 6));
        register_rock(&mut cave_shape, (503, 4), (502, 4));
        register_rock(&mut cave_shape, (502, 4), (502, 9));
        register_rock(&mut cave_shape, (502, 9), (494, 9));

        let parsed_cave = parse_cave_shape(input_buf).unwrap();

        assert_eq!(cave_shape, parsed_cave);
    }

    #[test]
    fn test_part_1() {
        let buf_input = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let mut cave_shape = parse_cave_shape(buf_input).unwrap();
        let total_sand = part_1(&mut cave_shape);

        assert_eq!(total_sand, 24);
    }

    #[test]
    fn test_part_2() {
        let buf_input = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let cave_shape = parse_cave_shape(buf_input).unwrap();
        let total_sand = part_2(cave_shape);

        assert_eq!(total_sand, 93);
    }
}
