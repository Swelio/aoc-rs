use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    path::PathBuf,
    result::Result,
};

use aoc_solver::{
    cli::{ChallengeFile, Cli},
    solver::UniversalSolver,
};
use aoc_traits::{
    challenge::{Challenge, ChallengeInput, InputName, Solution},
    DynamicSolver,
};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let solver = UniversalSolver::default();

    let challenges = open_challenge_inputs(cli.into_challenges());
    let solutions = resolve_challenges(&solver, challenges);
    let json_solutions =
        serde_json::to_string(&solutions).expect("serialization should always work");
    print!("{json_solutions}");
}

fn open_challenge_inputs(
    challenges: impl Iterator<Item = ChallengeFile>,
) -> impl Iterator<Item = Result<Challenge, Solution>> {
    let mut inputs: HashMap<PathBuf, ChallengeInput> = HashMap::new();

    challenges.map(move |challenge_file| -> Result<Challenge, Solution> {
        let input_name = InputName::new(challenge_file.file().display().to_string());
        let input = match inputs.entry(challenge_file.file().to_path_buf()) {
            Entry::Occupied(occupied_entry) => occupied_entry.get().to_owned(),
            Entry::Vacant(vacant_entry) => {
                let content = fs::read_to_string(challenge_file.file()).map_err(|err| {
                    Solution::new(
                        challenge_file.identity(),
                        input_name.to_owned(),
                        Err(err.into()),
                    )
                })?;
                let input = ChallengeInput::from(content);
                vacant_entry.insert(input).to_owned()
            }
        };

        let challenge = Challenge::new(challenge_file.identity(), input_name, input);
        Ok(challenge)
    })
}

fn resolve_challenges(
    solver: &impl DynamicSolver,
    challenges: impl Iterator<Item = Result<Challenge, Solution>>,
) -> Vec<Solution> {
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
}
