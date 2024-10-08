use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    path::{Path, PathBuf},
    result::Result,
};

use aoc_solver::UniversalSolver;
use aoc_traits::{
    challenge::{Challenge, ChallengeInput, Identity, InputName, Solution},
    DynamicSolver,
};
use clap::Parser;
use parser::parse_year;

#[derive(clap::Parser)]
#[command(version, author, about, arg_required_else_help(true))]
struct Solver {
    #[arg(short, long = "challenge", num_args = 1.., value_parser = parse_year, help = "<year>-<day>-<part>:<input-path>")]
    challenges: Vec<(Identity, PathBuf)>,
}

fn main() {
    let Solver { challenges } = Solver::parse();
    let solver = UniversalSolver::default();
    let mut inputs: HashMap<&Path, ChallengeInput> = HashMap::new();

    let challenges = challenges
        .iter()
        .map(|(identity, path)| -> Result<Challenge, Solution> {
            let input_name = InputName::new(path.display().to_string());

            let raw_input = match inputs.entry(path.as_path()) {
                Entry::Occupied(occupied_entry) => occupied_entry.get().to_owned(),
                Entry::Vacant(vacant_entry) => {
                    let content = match fs::read_to_string(path) {
                        Err(err) => {
                            return Err(Solution::new(*identity, input_name, Err(err.into())))
                        }
                        Ok(content) => content,
                    };
                    let input = ChallengeInput::from(content);
                    vacant_entry.insert(input).to_owned()
                }
            };

            Ok(Challenge::new(*identity, input_name, raw_input))
        });
    let solutions = {
        let mut unordered_solutions = challenges
            .map(|challenge| -> Solution {
                match challenge {
                    Ok(challenge) => solver.resolve(challenge),
                    Err(failed_preparation) => failed_preparation,
                }
            })
            .collect::<Vec<_>>();
        unordered_solutions.sort_by_key(|solution| solution.identity());
        unordered_solutions
    };
    let json_solutions =
        serde_json::to_string(&solutions).expect("serialization should always work");
    print!("{json_solutions}");
}

mod parser {
    use std::path::PathBuf;

    use aoc_traits::challenge::Identity;
    use winnow::{ascii::digit1, error::ContextError, seq, token::take_till, Parser};

    pub fn parse_year(input: &str) -> Result<(Identity, PathBuf), clap::Error> {
        let value_err = |err: String| clap::Error::raw(clap::error::ErrorKind::InvalidValue, err);
        let (year, day, part, path): (u16, u8, u8, PathBuf) =
            seq!(digit1::<_, ContextError>.parse_to(), _: '-', digit1.parse_to(), _: '-', digit1.parse_to(), _: ':', take_till(1.., [' ', ]).parse_to())
                .parse(input)
                .map_err(|err| value_err(err.to_string()))?;

        Ok((
            Identity::try_new(year, day, part).map_err(|err| value_err(err.to_string()))?,
            path,
        ))
    }
}
