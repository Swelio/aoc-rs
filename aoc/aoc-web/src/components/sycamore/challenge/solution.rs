use aoc_core::{
    archetypes::DaySolution,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
};
use sycamore::prelude::*;

#[component]
pub fn ChallengeSolution(solution: DaySolution<(Solution<Part1>, Solution<Part2>)>) -> View {
    view! {
        p {
            "Day: " (solution.identity.to_string())
        }
        p {
            "Part 1: " (solution.solutions.0.to_string())
        }
        p {
            "Part 2: " (solution.solutions.1.to_string())
        }
    }
}
