use nutype::nutype;

#[nutype(derive(Debug, Display, Clone, Copy, PartialEq, Eq, Serialize, Deserialize))]
pub struct Flag(i64);
