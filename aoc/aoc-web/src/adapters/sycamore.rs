use aoc_core::{
    archetypes::DaySolution,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
};

use crate::{components::sycamore::Challenge, ports::WasmRender};

pub struct App;

impl WasmRender for App {
    fn render(&self) {
        sycamore::render(
            Challenge::<year_2015::YearInput<DaySolution<(Solution<Part1>, Solution<Part2>)>>>,
        );
    }
}
