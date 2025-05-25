use std::fmt::Debug;

use aoc_core::{
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
    days::{day_01, day_02, day_03},
};

proptest! {
    #[test]
    fn test_part_1((input, expected) in year_parameters::<Part1>()) {
        let input = YearInput::try_parse(&input).unwrap();
        let solution: Solution<Part1> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

proptest! {
    #[test]
    fn test_part_2((input, expected) in year_parameters::<Part2>()) {
        let input = YearInput::try_parse(&input).unwrap();
        let solution: Solution<Part2> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

pub(crate) fn year_parameters<P: ?Sized + Debug + 'static>()
-> impl Strategy<Value = (TextInput, Solution<P>)>
where
    day_01::tests::Strategizer: SolutionStrategy<P>,
    day_02::tests::Strategizer: SolutionStrategy<P>,
    day_03::tests::Strategizer: SolutionStrategy<P>,
{
    prop_oneof![
        part_parameters::<P>(&day_01::tests::Strategizer).boxed(),
        part_parameters::<P>(&day_02::tests::Strategizer).boxed(),
        part_parameters::<P>(&day_03::tests::Strategizer).boxed(),
    ]
}
