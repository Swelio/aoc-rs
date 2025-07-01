mod failure;
mod form;
mod resolution;
mod solution;

use aoc_core::{
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
};
use sycamore::{
    prelude::*,
    web::{Transition, create_isomorphic_resource},
};

use crate::{adapters::sycamore::ChallengeId, error::WebResult};

use failure::ChallengeFailure;
use form::ChallengeForm;
use resolution::ChallengeResolution;
use solution::ChallengeSolution;

#[component(inline_props)]
pub fn Challenge<S>(id: ChallengeId, solver: S) -> View
where
    S: AsyncFn(TextInput) -> WebResult<DaySolution<(Solution<Part1>, Solution<Part2>)>>
        + Copy
        + 'static,
{
    let input = create_signal(None::<TextInput>);
    let result = create_isomorphic_resource(on(input, move || async move {
        match input.get_clone() {
            None => None,
            Some(input) => Some(solver(input).await),
        }
    }));

    let submit = move |form_input: String| {
        if input.with_untracked(|value| value.is_some()) {
            return;
        }

        if let Ok(form_input) = form_input.try_into() {
            input.set(Some(form_input));
        }
    };

    view! {
        Transition(fallback=ChallengeResolution) {
            (match result.get_clone() {
                Some(None) | None => view! { ChallengeForm(id=id, submit=submit) },
                Some(Some(Err(err))) => ChallengeFailure(err),
                Some(Some(Ok(solution))) => ChallengeSolution(solution)
            })
        }
    }
}
