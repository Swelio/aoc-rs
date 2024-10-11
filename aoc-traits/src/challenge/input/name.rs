use nutype::nutype;

#[nutype(
    sanitize(trim),
    derive(
        Debug,
        Display,
        Clone,
        AsRef,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord
    )
)]
pub struct InputName(String);
