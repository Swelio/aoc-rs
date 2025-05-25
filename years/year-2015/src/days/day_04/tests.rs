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
use proptest::{prelude::*, sample::select};

use super::Input;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2))]
    #[test]
    fn test_part_1((input, expected) in part_parameters::<Part1>(&Strategizer)) {
        let input = Input::try_parse(&input).unwrap();
        let solution: Solution<Part1> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

pub(crate) struct Strategizer;

impl SolutionStrategy<Part1> for Strategizer {
    fn strategy(&self) -> impl Strategy<Value = (TextInput, Solution<Part1>)> {
        select(&[("abcdef", "609043"), ("pqrstuv", "1048970")]).prop_map(|(input, expected)| {
            (
                TextInput::try_new(input).unwrap(),
                Solution::try_new(expected).unwrap(),
            )
        })
    }
}

impl SolutionStrategy<Part2> for Strategizer {
    fn strategy(&self) -> impl Strategy<Value = (TextInput, Solution<Part2>)> {
        select(&[("", "")]).prop_map(|(input, expected)| {
            (
                TextInput::try_new(input).unwrap(),
                Solution::try_new(expected).unwrap(),
            )
        })
    }
}
