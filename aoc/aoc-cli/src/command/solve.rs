use std::{
    fs,
    path::{Path, PathBuf},
};

use aoc_core::{
    archetypes::DaySolution,
    components::{Solution, TextInput, parts::Part1},
    ports::{Challenge, Parser},
};
use indicatif::{ProgressIterator, ProgressStyle};
use year_2015::days::day_01::Day01Input;

type RunSolution = Solution<Part1>;

pub fn run(files: &[PathBuf]) -> anyhow::Result<()> {
    let progress_style = ProgressStyle::with_template(
        "[{per_sec}/{elapsed}/{eta}] {wide_bar} {human_pos}/{human_len} {msg}",
    )?;
    let total_files = files.len() as u64;
    let solutions = files
        .iter()
        .progress_count(total_files)
        .with_style(progress_style)
        .map(solve_file)
        .collect::<anyhow::Result<Vec<_>>>()?;
    let output = serde_json::to_string_pretty(&solutions)?;

    println!("{output}");

    Ok(())
}

fn solve_file<P: AsRef<Path>>(path: P) -> anyhow::Result<DaySolution<RunSolution>> {
    let content = fs::read_to_string(path)?;
    let input = TextInput::try_new(&content)?;
    let challenge: Box<dyn Challenge<RunSolution>> = Box::new(Day01Input::try_parse(input)?);
    let solution = challenge.solve()?;

    Ok(DaySolution::<RunSolution>::new(2015, 1, solution))
}
