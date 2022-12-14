#![deny(clippy::all)]

use std::fs;
use std::io::{Read, Seek};

use utils::CodeSolution;

mod day_05_parsers;
mod day_07_parsers;
mod day_11_parser;
mod day_13_parser;
mod days;

pub fn run_solution<I>(day_number: u8, input: I) -> Result<(), Box<dyn std::error::Error>>
where
    I: Read + Seek,
{
    match day_number {
        1 => days::day_01::DailySolution::run(input),
        2 => days::day_02::DailySolution::run(input),
        3 => days::day_03::DailySolution::run(input),
        4 => days::day_04::DailySolution::run(input),
        5 => days::day_05::DailySolution::run(input),
        6 => days::day_06::DailySolution::run(input),
        7 => days::day_07::DailySolution::run(input),
        8 => days::day_08::DailySolution::run(input),
        9 => days::day_09::DailySolution::run(input),
        10 => days::day_10::DailySolution::run(input),
        11 => days::day_11::DailySolution::run(input),
        12 => days::day_12::DailySolution::run(input),
        13 => days::day_13::DailySolution::run(input),
        14 => days::day_14::DailySolution::run(input),
        _ if (1..=25).contains(&day_number) => todo!(),
        _ => unreachable!(),
    }
}

#[derive(clap::Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
pub struct Command {
    /// Day solution to run.
    #[clap(short = 'd', long, required = true)]
    running_day: u8,
    /// Path of the input file to use.
    #[clap(short = 'f', long, required = true)]
    input_file: String,
}

impl Command {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        run_solution(self.running_day, fs::File::open(self.input_file)?)
    }
}
