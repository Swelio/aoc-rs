use crate::{components::Solution, error::SantaResult};

use super::Input;

pub trait Solver<I: Input> {
    type Output;

    fn solve(&self, input: &I) -> SantaResult<Solution<I, Self::Output>>;
}
