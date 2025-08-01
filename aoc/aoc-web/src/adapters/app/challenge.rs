mod failure;
mod loading;
mod solution;

use aoc_core::views::SolutionView;
use sycamore::prelude::*;

use crate::error::WebResult;

use failure::ChallengeFailure;
use loading::LoadingChallenge;
use solution::ChallengeSolution;

#[component(inline_props)]
pub fn ChallengeView(solution: ReadSignal<Option<WebResult<SolutionView>>>) -> View {
    view! {
        (match solution.get_clone() {
            None => LoadingChallenge(),
            Some(Err(err)) => ChallengeFailure(err),
            Some(Ok(solution)) => ChallengeSolution(solution),
        })
    }
}
