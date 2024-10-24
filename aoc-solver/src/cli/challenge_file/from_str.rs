use std::str::FromStr;

use aoc_traits::{
    challenge::{identity_raw_parser, Identity},
    error::AocError,
};
use winnow::{combinator::separated_pair, token::take_while, Parser};

use super::ChallengeFile;

impl FromStr for ChallengeFile {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let ((year, day, part), file) = separated_pair(
            identity_raw_parser,
            ':',
            take_while(1.., |_| true).parse_to(),
        )
        .parse(input)?;

        let identity = Identity::try_new(year, day, part)?;

        Ok(Self::new(identity, file))
    }
}
