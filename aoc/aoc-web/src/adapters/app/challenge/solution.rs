use aoc_core::views::SolutionView;
use sycamore::prelude::*;

#[component]
pub fn ChallengeSolution(solution: SolutionView) -> View {
    let solutions_view = solution
        .solutions
        .into_iter()
        .map(|part| view! { li(class="solution-item") { (part.to_string()) } })
        .collect::<Vec<_>>();

    view! {
        p {
            "Day: " (solution.identity.to_string())
        }
        ul(class="solutions") {
            (solutions_view)
        }
    }
}
