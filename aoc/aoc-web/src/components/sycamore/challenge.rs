use aoc_core::{
    SantaError,
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
    ports::{Challenge, Parser},
};
use sycamore::{prelude::*, web::events::KeyboardEvent};

use crate::error::WebResult;

#[component]
pub fn Challenge<S>() -> View
where
    for<'a> S: Parser<&'a TextInput>
        + Challenge<DaySolution<(Solution<Part1>, Solution<Part2>)>>
        + 'static,
{
    let result = create_signal(None);
    let submission = move |input: String| {
        if result.with(|value| value.is_some()) {
            return;
        }

        result.set(Some(submit_challenge::<S>(&input)));
    };

    view! {
        (match result.get_clone() {
        None => ChallengeInput(submission),
        Some(Err(err)) => {
            view! {
                p { "Error occurred while processing challenge input: " (err.to_string()) }
            }
        }
        Some(Ok(solution)) => view! {
            p {
                "Day: " (solution.identity.to_string())
            }
            p {
                "Part 1: " (solution.solutions.0.to_string())
            }
            p {
                "Part 2: " (solution.solutions.1.to_string())
            }
        },
    })
    }
}

#[component]
fn ChallengeInput<F>(submit: F) -> View
where
    F: Fn(String) + Copy + 'static,
{
    let input = create_signal(String::new());
    let on_keydown = move |ev: KeyboardEvent| {
        (ev.key() == "Enter" && (ev.meta_key() || ev.ctrl_key()) && !input.with(String::is_empty))
            .then(|| {
                submit(input.take());
            });
    };

    view! {
        div {
            "New challenge:"
            textarea(bind:value=input, on:keydown=on_keydown, enterkeyhint="send")
        }
    }
}

fn submit_challenge<S>(input: &str) -> WebResult<DaySolution<(Solution<Part1>, Solution<Part2>)>>
where
    for<'a> S: Parser<&'a TextInput>
        + Challenge<DaySolution<(Solution<Part1>, Solution<Part2>)>>
        + 'static,
{
    let input = TextInput::try_new(input)?;
    let input = S::try_parse(&input).map_err(SantaError::from)?;

    Ok(S::solve(&input)?)
}
