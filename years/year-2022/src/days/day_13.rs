//! Instructions are there: https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
use std::error::Error;
use std::io::{BufRead, BufReader, Lines, Read, Seek, SeekFrom};

use utils::{CodeSolution, SantaError};

use crate::day_13_parser::{parse_packet, parse_packet_pairs, Packet};

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let mut input = BufReader::new(input);

        let total_ordered_pairs = part_1(&mut input)?;
        println!("Total ordered pairs: {}", total_ordered_pairs);

        input.seek(SeekFrom::Start(0))?;

        let decoder_key = part_2(input)?;
        println!("Decoder key is: {}", decoder_key);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<u32, Box<dyn Error>>
where
    I: BufRead,
{
    let mut lines = input.lines();
    let mut result = 0;
    let mut index = 0;

    while let Ok(Some(chunk)) = get_pair(&mut lines) {
        let (left_pair, right_pair) = parse_packet_pairs(&chunk)?;

        index += 1;

        match left_pair.cmp(&right_pair) {
            Ordering::Less => result += index,
            Ordering::Equal => {
                return Err(SantaError::InvalidInput(
                    "Pairs are strictly equals, unexpected.".to_string(),
                )
                .into())
            }
            Ordering::Greater => {}
        }
    }

    Ok(result)
}

fn part_2<I>(input: I) -> Result<u32, Box<dyn Error>>
where
    I: BufRead,
{
    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    ];

    // Packets list with the two additional divider packets
    let mut packets = divider_packets.clone();

    for line in input.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let packet = parse_packet(&line)?;
        packets.push(packet);
    }

    packets.sort();

    let mut total = 1;

    for packet in divider_packets {
        let index = packets.iter().position(|p| &packet == p).unwrap();
        total *= index as u32 + 1;
    }

    Ok(total)
}

fn get_pair<I>(input: &mut Lines<I>) -> Result<Option<String>, Box<dyn Error>>
where
    I: BufRead,
{
    let mut result = String::new();

    for _ in 0..2 {
        let new_line = match input.next() {
            Some(line) => line?,
            None => {
                return if result.is_empty() {
                    Ok(None)
                } else {
                    Err(
                        SantaError::InvalidInput("Missing second line in the pair.".to_string())
                            .into(),
                    )
                }
            }
        };
        result.push_str(&new_line);
        result.push('\n');
    }

    // Consume the blank line
    let _ = input.next();

    Ok(Some(result))
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_part_1() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let result = part_1(input_cursor).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let decoder_key = part_2(input_cursor).unwrap();

        assert_eq!(decoder_key, 140);
    }
}
