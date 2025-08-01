use aoc_core::components::TextInput;

use crate::error::WebError;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::AsRef, derive_more::Display,
)]
#[as_ref(str)]
pub struct ChallengeInput(String);

impl ChallengeInput {
    pub fn new<I: ToString>(input: I) -> Self {
        Self(input.to_string())
    }
}

impl TryFrom<ChallengeInput> for TextInput {
    type Error = WebError;

    fn try_from(input: ChallengeInput) -> Result<Self, Self::Error> {
        Self::try_from(input.0).map_err(WebError::from)
    }
}
