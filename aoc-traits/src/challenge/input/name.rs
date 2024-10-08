use nutype::nutype;

#[nutype(
    sanitize(trim),
    derive(Debug, Display, Clone, AsRef, Serialize, Deserialize)
)]
pub struct InputName(String);
