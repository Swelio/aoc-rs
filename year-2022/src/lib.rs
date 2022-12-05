#![deny(clippy::all)]

use std::fs;
use std::io::Read;

use utils::CodeSolution;

mod days;

pub fn run_solution<I>(day_number: u8, input: I) -> Result<(), Box<dyn std::error::Error>>
where
    I: Read,
{
    match day_number {
        1 => days::day_01::DayOne::run(input),
        _ if 0 < day_number && day_number <= 25 => todo!(),
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