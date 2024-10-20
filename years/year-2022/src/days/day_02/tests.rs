#![cfg(test)]

use aoc_traits::prelude::*;

use crate::Year2022Solver;

const INPUT: &str = include_str!("input.txt");
const PART_01_SOLUTION: i64 = 15;
const PART_02_SOLUTION: i64 = 12;

#[test]
fn test_part_1() {
    let solution =
        <Year2022Solver as Solver<Year2022, Day02, Part01>>::solve(&Year2022Solver, INPUT).unwrap();
    assert_eq!(solution.into_inner(), PART_01_SOLUTION);
}

#[test]
fn test_part_2() {
    let solution =
        <Year2022Solver as Solver<Year2022, Day02, Part02>>::solve(&Year2022Solver, INPUT).unwrap();
    assert_eq!(solution.into_inner(), PART_02_SOLUTION);
}
