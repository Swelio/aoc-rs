use std::fmt::Debug;

use aoc_core::{
    archetypes::DaySolution,
    components::{
        Solution, TextInput,
        parts::{Part1, Part2},
    },
    ports::{
        Challenge, Parser,
        tests::{SolutionStrategy, part_parameters},
    },
};
use proptest::prelude::*;

use crate::{
    YearInput,
    days::{day_01, day_02},
};

proptest! {
    #[test]
    fn test_part_1((input, expected) in year_parameters::<Part1>()) {
        let input = YearInput::try_parse(input).unwrap();
        let solution: DaySolution<Solution<Part1>> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

proptest! {
    #[test]
    fn test_part_2((input, expected) in year_parameters::<Part2>()) {
        let input = YearInput::try_parse(input).unwrap();
        let solution: DaySolution<Solution<Part2>> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

pub(crate) fn year_parameters<P: ?Sized + Debug + 'static>()
-> impl Strategy<Value = (TextInput, DaySolution<Solution<P>>)>
where
    day_01::tests::Strategizer: SolutionStrategy<P>,
    day_02::tests::Strategizer: SolutionStrategy<P>,
{
    prop_oneof![
        (
            Just(1u8),
            part_parameters::<P>(&day_01::tests::Strategizer).boxed(),
        ),
        (
            Just(2u8),
            part_parameters::<P>(&day_02::tests::Strategizer).boxed(),
        ),
    ]
    .prop_map(|(day_num, (input, expected))| (input, DaySolution::new(2015, day_num, expected)))
}
