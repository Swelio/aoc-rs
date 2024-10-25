#![cfg(test)]

use aoc_traits::prelude::*;

use crate::Year2022Solver;

const INPUT: &str = include_str!("input.txt");
const PART_01_SOLUTION: &str = "CMZ";
const PART_02_SOLUTION: &str = "MCD";

#[test]
fn test_part_1() {
    let solution =
        <Year2022Solver as Solver<Year2022, Day04, Part01>>::solve(&Year2022Solver, INPUT).unwrap();
    assert_eq!(
        solution.into_inner(),
        FlagKind::Str(PART_01_SOLUTION.to_string())
    );
}

#[test]
fn test_part_2() {
    let solution =
        <Year2022Solver as Solver<Year2022, Day04, Part02>>::solve(&Year2022Solver, INPUT).unwrap();
    assert_eq!(
        solution.into_inner(),
        FlagKind::Str(PART_02_SOLUTION.to_string())
    );
}
