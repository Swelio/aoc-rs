use aoc_core::views::SolutionView;
use sycamore::{futures::spawn_local, prelude::*};

use crate::{
    components::challenge::{ChallengeId, ChallengeInput},
    error::WebResult,
    ports::SolvingService,
};

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Into)]
pub struct Challenge {
    id: ChallengeId,
    input: ChallengeInput,
    solution: Signal<Option<WebResult<SolutionView>>>,
}

impl Challenge {
    pub fn new<S>(input: ChallengeInput, solver: S) -> Self
    where
        S: SolvingService,
    {
        let id = ChallengeId::new();
        let solutions = create_signal(None);

        spawn_local({
            let input = input.clone();

            async move {
                let solution = solver.solve(input).await;
                solutions.set(Some(solution));
            }
        });

        Self {
            id,
            input,
            solution: solutions,
        }
    }

    pub fn get_id(&self) -> ChallengeId {
        self.id
    }

    pub fn get_input(&self) -> &ChallengeInput {
        &self.input
    }

    pub fn get_solution(&self) -> ReadSignal<Option<WebResult<SolutionView>>> {
        *self.solution
    }
}
