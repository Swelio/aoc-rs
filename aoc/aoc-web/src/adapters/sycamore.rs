use aoc_core::{
    SantaError,
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
    ports::{Challenge, Parser},
};
use sycamore::prelude::*;
use uuid::Uuid;

use crate::{components::sycamore::Challenge, error::WebResult, ports::WasmRender};

pub struct App;

impl WasmRender for App {
    fn render(&self) {
        sycamore::render(AppComponent);
    }
}

#[component]
fn AppComponent() -> View {
    let challenges: Signal<Vec<ChallengeId>> = create_signal(vec![ChallengeId::new(); 14]);
    let rendered_challenges = view! {
        ul(class="challenges") {
            Keyed(
                list=challenges,
                view=|id| view! {
                    li(class="challenge") { Challenge(id=id, solver=solve) }
                },
                key=|id| *id,
            )
        }
    };

    view! {
        div(id="app") {
            (rendered_challenges)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub(crate) struct ChallengeId(Uuid);

impl ChallengeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

async fn solve(input: TextInput) -> WebResult<DaySolution<(Solution<Part1>, Solution<Part2>)>> {
    let input =
        year_2015::YearInput::<DaySolution<(Solution<Part1>, Solution<Part2>)>>::try_parse(&input)
            .map_err(SantaError::from)?;

    Ok(input.solve()?)
}
