use std::{
    fs,
    path::{Path, PathBuf},
};

use aoc_core::{
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
    ports::{Challenge, Parser},
};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

type RunSolution = (Solution<Part1>, Solution<Part2>);

pub fn run(files: &[PathBuf]) -> anyhow::Result<()> {
    let progress_style = ProgressStyle::with_template(
        "[{per_sec}/{elapsed}/{eta}] {wide_bar} {human_pos}/{human_len} {msg}",
    )?;
    let total_files = files.len() as u64;
    let (solutions, _errors) = files
        .iter()
        .progress_count(total_files)
        .with_style(progress_style)
        .map(solve_file)
        .partition_result::<Vec<_>, Vec<_>, _, _>();
    let output = serde_json::to_string_pretty(&solutions)?;

    println!("{output}");

    Ok(())
}

fn solve_file<P: AsRef<Path>>(path: P) -> anyhow::Result<FileSolution> {
    let file = path.as_ref().to_path_buf();
    let content = fs::read_to_string(path)?;
    let input = TextInput::try_new(&content)?;
    let challenge: Box<dyn Challenge<DaySolution<RunSolution>>> =
        Box::new(year_2015::YearInput::try_parse(input)?);

    let solutions = challenge.solve()?;
    let result = FileSolution { file, solutions };

    Ok(result)
}

#[derive(Debug, serde::Serialize)]
struct FileSolution {
    file: PathBuf,
    #[serde(flatten)]
    solutions: DaySolution<RunSolution>,
}
