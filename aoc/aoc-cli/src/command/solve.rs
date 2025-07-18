use std::{
    fmt::Debug,
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
    views::SolutionView,
};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

type RunSolution = (Solution<Part1>, Solution<Part2>);

pub fn run(files: &[PathBuf]) -> anyhow::Result<()> {
    let progress_style = ProgressStyle::with_template(
        "[{per_sec}/{elapsed}/{eta}] {wide_bar} {human_pos}/{human_len} {msg}",
    )?;
    let total_files = files.len() as u64;
    let (solutions, errors) = files
        .iter()
        .map(|path| {
            solve_file(path).map_err(|err| FileOutput {
                file: path.to_path_buf(),
                output: SolverError::new(err),
            })
        })
        .progress_count(total_files)
        .with_style(progress_style)
        .partition_result::<Vec<_>, Vec<_>, _, _>();
    let output = SolverOutput { solutions, errors };
    let render = serde_json::to_string_pretty(&output)?;

    println!("{render}");

    Ok(())
}

fn solve_file<P: AsRef<Path>>(path: P) -> anyhow::Result<FileOutput<SolutionView>> {
    let file = path.as_ref().to_path_buf();
    let content = fs::read_to_string(path)?;
    let input = TextInput::try_new(&content)?;
    let challenge: Box<dyn Challenge<DaySolution<RunSolution>>> =
        Box::new(year_2015::YearInput::<DaySolution<RunSolution>>::try_parse(
            &input,
        )?);
    let solution = challenge.solve()?;

    Ok(FileOutput {
        file,
        output: solution.into(),
    })
}

#[derive(Debug, serde::Serialize)]
struct SolverOutput {
    solutions: Vec<FileOutput<SolutionView>>,
    errors: Vec<FileOutput<SolverError>>,
}

#[derive(Debug, serde::Serialize)]
struct SolverError {
    error: String,
}

#[derive(Debug, serde::Serialize)]
struct FileOutput<S>
where
    S: Debug + serde::Serialize,
{
    file: PathBuf,
    #[serde(flatten)]
    output: S,
}

impl SolverError {
    pub fn new<T: ToString>(err: T) -> Self {
        Self {
            error: err.to_string(),
        }
    }
}
