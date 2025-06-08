#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    derive_more::Constructor,
    derive_more::Display,
)]
pub struct Year(i32);
