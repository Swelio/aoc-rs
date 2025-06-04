use std::borrow::Cow;

use proptest::{
    char::range,
    prelude::{Just, Strategy},
    prop_compose, prop_oneof, proptest,
    sample::select,
};

use crate::days::day_05::solver::part_2::parse_repeated_letter;

use super::part_2::parse_valid_pairs;

proptest! {
     #[test]
     fn test_part_2_pairs(
        (input, valid) in prop_oneof![
            unoverlapping_letters_pair_builder().prop_map(|input| (Cow::from(input), true)),
            overlapping_letters_pair_builder().prop_map(|input| (Cow::from(input), false)),
            select(&[("xyxy", true), ("aabcdefgaa", true), ("aaa", false)]).prop_map(|(input, is_nice)| (input.into(), is_nice))
        ]) {
        let result = parse_valid_pairs(&input);
        println!("{result:?}");
        assert_eq!(!result.is_empty(), valid);
     }
}

proptest! {
     #[test]
     fn test_part_2_letter((input, valid) in prop_oneof![
        repeated_letter_builder().prop_map(|input| (Cow::from(input), true)),
        select(&[("xyx", true), ("abcdefeghi", true), ("aaa", true), ("abc", false)]).prop_map(|(input, is_nice)| (input.into(), is_nice))
        ]) {
        let result = parse_repeated_letter(&input);
        println!("{result:?}");
        assert_eq!(!result.is_empty(), valid);
     }
}

prop_compose! {
    pub fn unoverlapping_letters_pair_builder()
        (first in range('a', 'z'), second in range('a', 'z'))
        (first in Just(first), second in Just(second), delimiter in range('a', 'z').prop_filter("delimiter must be different from paired letters in order to ensure no overlapping", move |delimiter| !&[first, second].contains(delimiter))) -> String {
            format!("{delimiter}{first}{second}{delimiter}{first}{second}{delimiter}")
        }
}

prop_compose! {
    pub fn overlapping_letters_pair_builder()
        (letter in range('a', 'z'))
        (letter in Just(letter), delimiter in range('a', 'z').prop_filter("delimiter must be different from paired letters in order to ensure no overlapping", move |delimiter| *delimiter != letter))
        -> String {
            format!("{letter}{letter}{letter}{delimiter}")
        }
}

prop_compose! {
    pub fn repeated_letter_builder()
    (letter in range('a', 'z'), separator in range('a', 'z'))
     -> String {
        format!("{letter}{separator}{letter}{letter}{separator}{letter}")
    }
}
