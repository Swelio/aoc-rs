use std::num::NonZeroU8;

use crate::{SantaError, SantaResult};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, derive_more::Display,
)]
#[display("{_0:02}")]
pub struct Day(NonZeroU8);

impl Day {
    pub fn try_new(number: u8) -> SantaResult<Self> {
        let inner = NonZeroU8::new(number).ok_or(SantaError::CannotBeZero)?;
        Ok(Self(inner))
    }
}
