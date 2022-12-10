//! Instructions are there: https://adventofcode.com/2022/day/6

use std::collections::{HashSet, VecDeque};
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

        let marker_index = part_1(&mut buf_input)?;
        println!("Packet marker found at index: {}", marker_index);

        buf_input.seek(SeekFrom::Start(0))?;

        let marker_index = part_2(buf_input)?;
        println!("Message marker found at index: {}", marker_index);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<usize, Box<dyn Error>>
where
    I: BufRead,
{
    find_unique_chunk_index(input, 4)
}

fn part_2<I>(input: I) -> Result<usize, Box<dyn Error>>
where
    I: BufRead,
{
    find_unique_chunk_index(input, 14)
}

fn find_unique_chunk_index<I>(input: I, chunk_size: usize) -> Result<usize, Box<dyn Error>>
where
    I: BufRead,
{
    let mut current_chunk = VecDeque::new();
    let mut bytes = input.bytes();

    for _ in 0..(chunk_size - 1) {
        let msg_char = bytes
            .next()
            .ok_or_else(|| SantaError::InvalidInput("Input is too short.".to_string()))??
            as char;
        current_chunk.push_back(msg_char);
    }

    for (index, msg_char) in bytes.enumerate() {
        let msg_char = msg_char? as char;

        if !current_chunk.contains(&msg_char) && chunk_all_unique(current_chunk.make_contiguous()) {
            // We add chunk_size to count chars in the previous loop (chunk_size - 1) + 1 to start from 1 instead of 0
            return Ok(index + chunk_size);
        }

        current_chunk.pop_front();
        current_chunk.push_back(msg_char);
    }

    Err(SantaError::InvalidInput("No marker in the received stream.".to_string()).into())
}

/// Check if all characters of the chunk are unique
fn chunk_all_unique(chunk: &[char]) -> bool {
    let chunk_set: HashSet<char> = chunk.iter().copied().collect();

    chunk.len() == chunk_set.len()
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use utils::setup_log;

    use super::*;

    const SAMPLE_INPUT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#;

    #[test]
    fn test_part_1() {
        setup_log();

        let mut lines = BufReader::new(Cursor::new(SAMPLE_INPUT)).lines();

        assert_eq!(
            part_1(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            7
        );
        assert_eq!(
            part_1(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            5
        );
        assert_eq!(
            part_1(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            6
        );
        assert_eq!(
            part_1(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            10
        );
        assert_eq!(
            part_1(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            11
        );
    }

    #[test]
    fn test_part_2() {
        setup_log();

        let mut lines = BufReader::new(Cursor::new(SAMPLE_INPUT)).lines();

        assert_eq!(
            part_2(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            19
        );
        assert_eq!(
            part_2(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            23
        );
        assert_eq!(
            part_2(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            23
        );
        assert_eq!(
            part_2(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            29
        );
        assert_eq!(
            part_2(BufReader::new(Cursor::new(lines.next().unwrap().unwrap()))).unwrap(),
            26
        );
    }
}
