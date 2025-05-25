use aoc_core::{
    ParsingError, ParsingResult, SantaResult,
    archetypes::DaySolution,
    components::TextInput,
    ports::{self, Challenge},
};
use aoc_macros::combine_parsers;

use crate::days::{day_01, day_02, day_03};

pub struct YearInput<S: ?Sized> {
    inner: Box<dyn Challenge<S>>,
    day: u8,
}

impl<S> YearInput<S> {
    fn new(inner: Box<dyn Challenge<S> + 'static>, day: u8) -> Self {
        Self { day, inner }
    }
}

impl<S> ports::Parser<&TextInput> for YearInput<S>
where
    day_01::Input: Challenge<S>,
    day_02::Input: Challenge<S>,
    day_03::Input: Challenge<S>,
{
    fn try_parse(input: &TextInput) -> ParsingResult<Self> {
        let parser = combine_parsers!(
            Box<dyn Challenge<S> + 'static>,
            day_01::Input,
            day_02::Input,
            day_03::Input
        );

        parser(input)
            .map(|solver: Box<dyn Challenge<S> + 'static>| YearInput::<S>::new(solver, 1))
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

impl<S> ports::Challenge<DaySolution<S>> for YearInput<S> {
    fn solve(&self) -> SantaResult<DaySolution<S>> {
        self.inner
            .solve()
            .map(|solutions| DaySolution::<S>::new(2015, self.day, solutions))
    }
}
