use aoc_core::{
    ParsingError, ParsingResult, SantaResult,
    components::TextInput,
    ports::{self, Challenge},
};
use aoc_macros::combine_parsers;

use crate::days::{day_01, day_02, day_03, day_04};

pub struct YearInput<S: ?Sized>(Box<dyn Challenge<S>>);

impl<S> YearInput<S> {
    fn new(inner: Box<dyn Challenge<S> + 'static>) -> Self {
        Self(inner)
    }
}

impl<S> ports::Parser<&TextInput> for YearInput<S>
where
    day_01::Input: Challenge<S>,
    day_02::Input: Challenge<S>,
    day_03::Input: Challenge<S>,
    day_04::Input: Challenge<S>,
{
    fn try_parse(input: &TextInput) -> ParsingResult<Self> {
        let parser = combine_parsers!(
            Box<dyn Challenge<S> + 'static>,
            day_01::Input,
            day_02::Input,
            day_03::Input,
            day_04::Input
        );

        parser(input)
            .map(YearInput::<S>::new)
            .map_err(|err| ParsingError::from(err.to_string()))
    }
}

impl<S> ports::Challenge<S> for YearInput<S> {
    fn solve(&self) -> SantaResult<S> {
        self.0.solve()
    }
}
