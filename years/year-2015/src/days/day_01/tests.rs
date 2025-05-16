use aoc_core::{
    components::TextInput,
    ports::{Parser, Solver},
};
use proptest::{prelude::*, sample::select};

use crate::solver::Year2015Solver;

use super::input::Day01Input;

proptest! {
    #[test]
    fn test_part_1((input, expected) in part_01_parameters()) {
        let input = Day01Input::try_parse(input).unwrap();
        let solution = Year2015Solver.solve(&input).unwrap();

        assert_eq!(*solution.part_01(), expected);
    }
}

#[test]
#[ignore = "require part 1"]
fn test_part_2() {
    todo!()
}

fn part_01_parameters() -> impl Strategy<Value = (TextInput, i32)> {
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
    .prop_map(|(input, expected)| (TextInput::try_new(input).unwrap(), expected))
}
