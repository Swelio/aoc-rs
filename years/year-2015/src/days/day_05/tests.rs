use std::borrow::Cow;

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

use super::{
    Input,
    solver::tests::{repeated_letter_builder, unoverlapping_letters_pair_builder},
};

proptest! {
    #[test]
    fn test_part_1((input, expected) in part_parameters::<Part1>(&Strategizer)) {
        let input = Input::try_parse(&input).unwrap();
        let solution: Solution<Part1> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

proptest! {
    #[test]
    fn test_part_2((input, expected) in part_parameters::<Part2>(&Strategizer)) {
        let input = Input::try_parse(&input).unwrap();
        let solution: Solution<Part2> = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

pub(crate) struct Strategizer;

impl SolutionStrategy<Part1> for Strategizer {
    fn strategy(&self) -> impl Strategy<Value = (TextInput, Solution<Part1>)> {
        select(&[
            ("ugknbfddgicrmopn", "1"),
            ("aaa", "1"),
            ("jchzalrnumimnmhp", "0"),
            ("haegwjzuvuyypxyu", "0"),
            ("dvszwmarrgswjxmb", "0"),
        ])
        .prop_map(|(input, expected)| {
            (
                TextInput::try_new(input).unwrap(),
                Solution::try_new(expected).unwrap(),
            )
        })
    }
}

impl SolutionStrategy<Part2> for Strategizer {
    fn strategy(&self) -> impl Strategy<Value = (TextInput, Solution<Part2>)> {
        prop_oneof![
            nice_string_builder().prop_map(|string| (Cow::from(string), "1")),
            select(&[
                ("qjhvhtzxzqqjkmpb", "1"),
                ("xxyxx", "1"),
                ("uurcxstgmygtbstg", "0"),
                ("ieodomkazucvgmuy", "0"),
                ("aaaa", "1"),
                ("aaabcb", "0"),
            ])
            .prop_map(|(string, response)| (string.into(), response))
        ]
        .prop_map(|(input, expected)| {
            (
                TextInput::try_new(&input).unwrap(),
                Solution::try_new(expected).unwrap(),
            )
        })
    }
}

prop_compose! {
    fn nice_string_builder()
    (pair in unoverlapping_letters_pair_builder(), repeated_letter in repeated_letter_builder())
    -> String {
        format!("{pair}{repeated_letter}")
    }
}
