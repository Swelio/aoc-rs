use crate::error::{SantaError, SantaResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::AsRef)]
#[as_ref(str)]
pub struct TextInput(String);

impl TextInput {
    pub fn try_new<I: ToString>(input: I) -> SantaResult<Self> {
        let input = input.to_string();

        if input.is_empty() {
            return Err(SantaError::EmptyInput);
        }

        Ok(Self(input))
    }
}

impl TryFrom<String> for TextInput {
    type Error = SantaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
