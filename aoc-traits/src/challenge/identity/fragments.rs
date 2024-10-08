use nutype::nutype;

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
        Hash,
        PartialOrd,
        Ord
    )
)]
pub struct Year(u16);

#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 31),
    derive(
        Clone,
        Copy,
        Debug,
        Display,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord
    )
)]
pub struct Day(u8);

#[nutype(
    validate(greater_or_equal = 1),
    derive(
        Clone,
        Copy,
        Debug,
        Display,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord
    )
)]
pub struct Part(u8);
