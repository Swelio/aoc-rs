use nutype::nutype;

#[nutype(derive(Clone, Copy, Debug, Display, Serialize, Deserialize, PartialEq, Eq))]
pub struct Solution(i64);
