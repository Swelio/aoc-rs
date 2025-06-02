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

mod part_2 {
    use std::collections::HashMap;

    use winnow::{
        Parser,
        combinator::{peek, repeat, seq, terminated},
        stream::AsChar,
        token::{any, take},
    };

    pub fn parse_valid_pairs<'input>(input: &'input str) -> Vec<&'input str> {
        let parser = |input: &mut &'input str| -> winnow::Result<Option<&'input str>> {
            if input.len() < 2 {
                take(input.len().max(1usize)).void().parse_next(input)?;
                return Ok(None);
            }

            if input.len() == 2 {
                return Ok(Some(take(2usize).parse_next(input)?));
            }

            let parse_slice =
                take(3usize).verify(|pair: &str| pair.chars().all(|c| c.is_ascii_alphabetic()));
            let current_slice = terminated(peek(parse_slice), any).parse_next(input)?;

            let current_pair = &current_slice[..2];
            let next_pair = &current_slice[1..];

            Ok((current_pair != next_pair).then_some(current_pair))
        };

        let pairs = repeat(0.., parser)
            .map(|pairs: Vec<Option<&'input str>>| {
                pairs
                    .into_iter()
                    .flatten()
                    .fold(HashMap::new(), |mut acc, current| {
                        acc.entry(current)
                            .and_modify(|e| {
                                *e += 1;
                            })
                            .or_insert(1usize);
                        acc
                    })
            })
            .parse(input)
            .expect("input already verified through parsing");

        pairs
            .into_iter()
            .filter_map(|(pair, count)| (count >= 2).then_some(pair))
            .collect()
    }

    pub fn parse_repeated_letter<'input>(input: &'input str) -> Vec<char> {
        let parser = |input: &mut &'input str| -> winnow::Result<Option<_>> {
            if input.len() < 3 {
                take(input.len().max(1usize)).void().parse_next(input)?;
                return Ok(None);
            }

            let mut parse_letter = |input: &mut &str| -> winnow::Result<_> {
                any.verify(|c: &char| c.is_alpha()).parse_next(input)
            };
            let parse_slice = seq!(parse_letter, _: any, parse_letter);
            let (first, second) = terminated(peek(parse_slice), any).parse_next(input)?;

            Ok((first == second).then_some(first))
        };

        repeat(0.., parser)
            .map(|letters: Vec<_>| letters.into_iter().flatten().collect::<Vec<_>>())
            .parse(input)
            .expect("input already verified through parsing")
    }
}

#[cfg(test)]
mod tests {
    use proptest::{proptest, sample::select};

    use crate::days::day_05::solver::part_2::parse_repeated_letter;

    use super::part_2::parse_valid_pairs;

    proptest! {
         #[test]
         fn test_part_2_pairs((input, valid) in select(&[("xyxy", true), ("aabcdefgaa", true), ("aaa", false)])) {
            let result = parse_valid_pairs(input);
            println!("{result:?}");
            assert_eq!(result.len() == 1, valid);
         }
    }

    proptest! {
         #[test]
         fn test_part_2_letter((input, valid) in select(&[("xyx", true), ("abcdefeghi", true), ("aaa", true), ("abc", false)])) {
            let result = parse_repeated_letter(input);
            println!("{result:?}");
            assert_eq!(result.len() == 1, valid);
         }
    }
}
