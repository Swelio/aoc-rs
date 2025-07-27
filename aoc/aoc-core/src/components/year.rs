use crate::SantaResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Into)]
pub struct Year(i32);

impl Year {
    pub fn try_new(number: i32) -> SantaResult<Self> {
        Ok(Self(number))
    }
}
