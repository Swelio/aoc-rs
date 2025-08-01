use aoc_core::{
    SantaError,
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
    ports::{Challenge as ChallengeSolver, Parser},
    views::SolutionView,
};
use async_trait::async_trait;

use crate::{components::challenge::ChallengeInput, error::WebResult, ports::SolvingService};

#[derive(Debug, Clone, Copy)]
pub struct WebSolver;

#[async_trait]
impl SolvingService for WebSolver {
    async fn solve(&self, input: ChallengeInput) -> WebResult<SolutionView> {
        let input = TextInput::try_from(input)?;
        let input =
            year_2015::YearInput::<DaySolution<(Solution<Part1>, Solution<Part2>)>>::try_parse(
                &input,
            )
            .map_err(SantaError::from)?;
        let solution = input.solve()?;

        Ok(solution.into())
    }
}
