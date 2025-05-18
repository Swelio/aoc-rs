use crate::components::{
    Solution,
    parts::{Part1, Part2},
};

#[derive(Debug, serde::Serialize)]
pub struct DaySolution<S> {
    year: i16,
    day: u8,
    solutions: S,
}

impl DaySolution<(Solution<Part1>, Solution<Part2>)> {
    pub fn new(year: i16, day: u8, solutions: (Solution<Part1>, Solution<Part2>)) -> Self {
        Self {
            year,
            day,
            solutions,
        }
    }
}

impl DaySolution<Solution<Part1>> {
    pub fn new(year: i16, day: u8, solutions: Solution<Part1>) -> Self {
        Self {
            year,
            day,
            solutions,
        }
    }
}

impl DaySolution<Solution<Part2>> {
    pub fn new(year: i16, day: u8, solutions: Solution<Part2>) -> Self {
        Self {
            year,
            day,
            solutions,
        }
    }
}
