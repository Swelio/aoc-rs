use nutype::nutype;

#[nutype(derive(Debug, Display, Serialize, Deserialize, PartialEq, Eq))]
pub struct Solution(i64);
