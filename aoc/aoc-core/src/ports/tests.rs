pub use solution::SolutionStrategy;

use std::fmt::Debug;

use proptest::prelude::Strategy;

use crate::components::{Solution, TextInput};

mod solution;

pub fn part_parameters<P>(
    strategizer: &impl SolutionStrategy<P>,
) -> impl Strategy<Value = (TextInput, Solution<P>)>
where
    P: ?Sized + Debug,
{
    strategizer.strategy()
}
