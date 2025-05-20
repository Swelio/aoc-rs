use std::fmt::Debug;

use proptest::prelude::Strategy;

use crate::components::{Solution, TextInput};

pub trait SolutionStrategy<P: ?Sized + Debug> {
    fn strategy(&self) -> impl Strategy<Value = (TextInput, Solution<P>)>;
}
