use std::str::FromStr;

use winnow::{ascii::digit1, error::ContextError, seq, Parser};

use crate::error::{AocError, AocResult};

use super::{Identity, IdentityError};

impl FromStr for Identity {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_identity(input)
    }
}

fn parse_identity(input: &str) -> AocResult<Identity> {
    let (year, day, part) =
            seq!(digit1::<_, ContextError>.parse_to(), _: '-', digit1.parse_to(), _: '-', digit1.parse_to())
                .parse(input).map_err(IdentityError::from)?;

    Identity::try_new(year, day, part)
}

impl TryFrom<String> for Identity {
    type Error = AocError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_parsing() {
        let input = "2015-03-02";
        let result = parse_identity(input).unwrap();
        assert_eq!(result, Identity::try_new(2015, 3, 2).unwrap());
    }

    #[test]
    fn string_serialization() {
        let identity = Identity::try_new(2015, 1, 2).unwrap();
        let serialized = Identity::from_str(&identity.to_string()).unwrap();

        assert_eq!(identity, serialized);
    }
}
