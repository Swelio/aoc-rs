use nutype::nutype;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Identity(Year, Day, Part);

impl Identity {
    pub fn new(year: Year, day: Day, part: Part) -> Self {
        Self(year, day, part)
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

#[nutype(
    validate(greater_or_equal = 2015),
    derive(
        Clone,
        Copy,
        Debug,
        Display,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        Hash
    )
)]
pub struct Year(u16);

#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 31),
    derive(Clone, Copy, Debug, Display, Serialize, Deserialize)
)]
pub struct Day(u8);

#[nutype(
    validate(greater_or_equal = 1),
    derive(Clone, Copy, Debug, Display, Serialize, Deserialize)
)]
pub struct Part(u8);
