use aoc_core::{
    SantaError, SantaResult,
    archetypes::DaySolution,
    components::TextInput,
    ports::{self, Challenge},
};
use winnow::{Parser, combinator::alt};

use crate::days::{day_01, day_02, day_03};

pub struct YearInput<S: ?Sized> {
    inner: Box<dyn Challenge<S>>,
    day: u8,
}

impl<S> YearInput<S> {
    fn new<I>(input: I, day: u8) -> Self
    where
        I: Challenge<S> + 'static,
    {
        Self {
            day,
            inner: Box::new(input),
        }
    }
}

impl<S> ports::Parser<TextInput> for YearInput<S>
where
    day_01::Input: Challenge<S>,
    day_02::Input: Challenge<S>,
    day_03::Input: Challenge<S>,
{
    fn try_parse(input: TextInput) -> SantaResult<Self> {
        parse_input::<S>
            .parse(input.as_ref())
            .map_err(|err| SantaError::ParsingError(anyhow::format_err!("{err}")))
    }
}

pub fn parse_input<S>(input: &mut &str) -> winnow::Result<YearInput<S>>
where
    day_01::Input: Challenge<S>,
    day_02::Input: Challenge<S>,
    day_03::Input: Challenge<S>,
{
    alt((
        day_01::parse_input.map(|input| YearInput::<S>::new(input, 1)),
        day_02::parse_input.map(|input| YearInput::<S>::new(input, 2)),
        day_03::parse_input.map(|input| YearInput::<S>::new(input, 3)),
    ))
    .parse_next(input)
}

impl<S> ports::Challenge<DaySolution<S>> for YearInput<S> {
    fn solve(&self) -> SantaResult<DaySolution<S>> {
        self.inner
            .solve()
            .map(|solutions| DaySolution::<S>::new(2015, self.day, solutions))
    }
}
