use std::fmt::Debug;

use aoc_core::{
    archetypes::DaySolution,
    components::{Solution, TextInput, parts::Part1},
    ports::{Challenge, Parser},
};
use proptest::prelude::*;

use crate::{YearInput, days::day_01};

proptest! {
    #[test]
    fn test_part_1((input, expected) in part_parameters::<Part1>()) {
        let input = YearInput::try_parse(input).unwrap();
        let solution = input.solve().unwrap();

        assert_eq!(solution, expected);
    }
}

#[test]
#[ignore = "require part 1"]
fn test_part_2() {
    todo!()
}

pub(crate) fn part_parameters<P: ?Sized + Debug>()
-> impl Strategy<Value = (TextInput, DaySolution<Solution<P>>)> {
    prop_oneof![day_01::tests::part_parameters::<P>()]
        .prop_map(|(input, expected)| (input, DaySolution::new(2015, 1, expected)))
}
