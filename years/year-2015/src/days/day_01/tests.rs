use std::fmt::Debug;

use aoc_core::{
    components::{Solution, TextInput, parts::Part1},
    ports::{Challenge, Parser},
};
use proptest::{prelude::*, sample::select};

use super::input::Day01Input;

proptest! {
    #[test]
    fn test_part_1((input, expected) in part_parameters::<Part1>()) {
        let input = Day01Input::try_parse(input).unwrap();
        let solution = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

#[test]
#[ignore = "require part 1"]
fn test_part_2() {
    todo!()
}

fn part_parameters<P: ?Sized + Debug>() -> impl Strategy<Value = (TextInput, Solution<P>)> {
    select(&[
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ])
    .prop_map(|(input, expected)| {
        (
            TextInput::try_new(input).unwrap(),
            Solution::try_new(expected).unwrap(),
        )
    })
}
