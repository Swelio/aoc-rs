use std::fmt::{self, Display};

use crate::{challenge::identity::IdentityError, error::AocResult};

use super::fragments::{Day, Part, Year};

#[derive(
    Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(into = "String")]
#[serde(try_from = "String")]
pub struct Identity(Year, Day, Part);

impl Identity {
    pub fn new(year: Year, day: Day, part: Part) -> Self {
        Self(year, day, part)
    }

    pub fn try_new(year: u16, day: u8, part: u8) -> AocResult<Self> {
        let year_res = Year::try_new(year);
        let day_res = Day::try_new(day);
        let part_res = Part::try_new(part);

        let errors = match (&year_res, &day_res, &part_res) {
            (Ok(year), Ok(day), Ok(part)) => return Ok(Self::new(*year, *day, *part)),
            _ => year_res
                .err()
                .map(|err| err.to_string())
                .into_iter()
                .chain(day_res.err().map(|err| err.to_string()))
                .chain(part_res.err().map(|err| err.to_string()))
                .collect(),
        };

        Err(IdentityError::Validation(errors).into())
    }

    pub fn as_tuple(&self) -> (u16, u8, u8) {
        (
            self.0.into_inner(),
            self.1.into_inner(),
            self.2.into_inner(),
        )
    }

    pub fn year(&self) -> Year {
        self.0
    }

    pub fn day(&self) -> Day {
        self.1
    }

    pub fn part(&self) -> Part {
        self.2
    }
}

impl Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year(), self.day(), self.part())
    }
}

impl From<Identity> for String {
    fn from(identity: Identity) -> Self {
        identity.to_string()
    }
}
