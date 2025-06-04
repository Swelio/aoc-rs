#[cfg(test)]
pub(super) mod tests;

mod part_2;

use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};
use regex::Regex;

use super::Input;

use self::part_2::{parse_repeated_letter, parse_valid_pairs};

impl Challenge<Solution<Part1>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        let forbidden_strings =
            Regex::new(r#"ab|cd|pq|xy"#).expect("must be correct at compile time");
        let vowels = Regex::new(r#"a|e|i|o|u"#).expect("must be correct at compile time");

        let nice_strings = self
            .into_iter()
            .filter(|line| {
                let match_vowels = || vowels.find_iter(line.as_ref()).count() >= 3;
                let contain_twins = || {
                    line.as_ref()
                        .as_bytes()
                        .windows(2)
                        .filter(|letters| letters[0] == letters[1])
                        .count()
                        >= 1
                };
                let reject_forbidden_string = || !forbidden_strings.is_match(line.as_ref());

                match_vowels() && contain_twins() && reject_forbidden_string()
            })
            .count();

        Solution::try_new(nice_strings)
    }
}

impl Challenge<Solution<Part2>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part2>> {
        let nice_strings = self
            .into_iter()
            .filter(|line| {
                let has_valid_pairs = || !parse_valid_pairs(line.as_ref()).is_empty();
                let has_repeated_letter = || !parse_repeated_letter(line.as_ref()).is_empty();

                has_valid_pairs() && has_repeated_letter()
            })
            .count();

        Solution::try_new(nice_strings)
    }
}
