use aoc_core::ports::Input;

use super::components::Direction;

#[derive(Debug, derive_more::Constructor, derive_more::IntoIterator)]
pub struct Day01Input {
    #[into_iterator(owned, ref)]
    directions: Vec<Direction>,
}

impl Input for Day01Input {
    const LABEL: &str = "year_2015/day_01";
}
