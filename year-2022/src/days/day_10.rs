//! Instructions are there: https://adventofcode.com/2022/day/10

use std::error::Error;
use std::io::{BufRead, BufReader, Lines, Read, Seek, SeekFrom};

use log::debug;

use utils::{CodeSolution, SantaError};

type InstructionDelay = u8;
type RegisterValue = i32;
type SignalStrength = u32;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let mut buf_input = BufReader::new(input);

        let total_signal_strength = part_1(&mut buf_input)?;
        println!("Signal strength is {}", total_signal_strength);

        buf_input.seek(SeekFrom::Start(0))?;

        let crt_img = part_2(buf_input)?;
        println!("CRT image:\n{}", crt_img);

        Ok(())
    }
}

fn part_1<I>(input: I) -> Result<SignalStrength, Box<dyn Error>>
where
    I: BufRead,
{
    let cpu = CPU::new(input);
    let mut total_strength = 0;

    for (cycle, register_value) in cpu.into_iter().enumerate().skip(19).step_by(40) {
        let register_value = register_value? as usize;
        let cycle = cycle + 1;
        debug!("Current cycle: {}", cycle);

        total_strength += cycle * register_value;
    }

    Ok(total_strength as SignalStrength)
}

fn part_2<I>(input: I) -> Result<String, Box<dyn Error>>
where
    I: BufRead,
{
    let cpu = CPU::new(input);
    let mut crt_img = String::new();

    for (cycle, sprite_pos) in cpu.into_iter().enumerate().take(240) {
        let sprite_pos = sprite_pos? as RegisterValue;
        let crt_pos = (cycle % 40) as RegisterValue;

        debug!(
            "Cycle {}, sprite position {} with crt pos {}",
            cycle, sprite_pos, crt_pos
        );

        // New line on CRT screen
        if cycle > 0 && crt_pos == 0 {
            crt_img.push('\n');
        }

        if crt_pos.abs_diff(sprite_pos) <= 1 {
            crt_img.push('#');
        } else {
            crt_img.push('.');
        }
    }

    Ok(crt_img)
}

#[derive(Copy, Clone)]
enum Instruction {
    AddX(RegisterValue),
    NoOp,
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            Ok(Self::NoOp)
        } else if value.starts_with("addx") {
            let op_value = value
                .split_once(' ')
                .ok_or_else(|| {
                    SantaError::InvalidInput(format!("Invalid cpu instruction '{}'", value))
                })?
                .1
                .parse()?;
            Ok(Self::AddX(op_value))
        } else {
            Err(SantaError::InvalidInput(format!("Unknown cpu instruction '{}'", value)).into())
        }
    }
}

struct CPU<B>
where
    B: BufRead,
{
    current_instruction: Option<(InstructionDelay, Instruction)>,
    eof: bool,
    instructions_stream: Lines<B>,
    register: RegisterValue,
}

impl<B> CPU<B>
where
    B: BufRead,
{
    fn new(instructions_buffer: B) -> Self {
        Self {
            current_instruction: None,
            eof: false,
            instructions_stream: instructions_buffer.lines(),
            register: 1,
        }
    }

    fn load_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        if self.current_instruction.is_none() {
            if let Some(instruction) = self.instructions_stream.next() {
                let new_instruction = Instruction::try_from(instruction?.as_str())?;

                match new_instruction {
                    Instruction::NoOp => self.current_instruction = Some((1, new_instruction)),
                    Instruction::AddX(_) => self.current_instruction = Some((2, new_instruction)),
                };
            } else {
                self.eof = true;
            }
        }

        Ok(())
    }

    fn run_instruction(&mut self) {
        if let Some((delay, instruction)) = &mut self.current_instruction {
            *delay -= 1;

            if *delay == 0 {
                match instruction {
                    Instruction::NoOp => {}
                    Instruction::AddX(value) => self.register += *value,
                }

                self.current_instruction = None;
            }
        }
    }
}

impl<B> Iterator for CPU<B>
where
    B: BufRead,
{
    type Item = Result<RegisterValue, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        if let Err(err) = self.load_instruction() {
            return Some(Err(err));
        }

        let item = self.register;
        self.run_instruction();

        Some(Ok(item))
    }
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../assets/day_10_test_input.txt");

    #[test]
    fn test_cpu() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let mut cpu = CPU::new(input_cursor).into_iter();

        assert_eq!(cpu.nth(9).unwrap().unwrap(), 8); // 10th element
        assert_eq!(cpu.nth(0).unwrap().unwrap(), 13); // 11th element
        assert_eq!(cpu.nth(8).unwrap().unwrap(), 21); // 20th element
        assert_eq!(cpu.nth(39).unwrap().unwrap(), 19); // 60th element
    }

    #[test]
    fn test_part_1() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let total_strength = part_1(input_cursor).unwrap();

        assert_eq!(total_strength, 13140);
    }

    #[test]
    fn test_part_2() {
        const SAMPLE_OUTPUT: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let crt_img = part_2(input_cursor).unwrap();

        for (line, sample_line) in crt_img.lines().zip(SAMPLE_OUTPUT.lines()) {
            assert_eq!(line, sample_line);
        }

        assert_eq!(crt_img, SAMPLE_OUTPUT);
    }
}
