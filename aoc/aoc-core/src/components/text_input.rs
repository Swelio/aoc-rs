use crate::error::{SantaError, SantaResult};

#[derive(Debug, Clone, derive_more::AsRef)]
#[as_ref(str)]
pub struct TextInput(String);

impl TextInput {
    pub fn try_new<I: AsRef<str> + ToString>(input: I) -> SantaResult<Self> {
        if input.as_ref().is_empty() {
            return Err(SantaError::EmptyInput);
        }

        Ok(Self(input.to_string()))
    }
}

impl TryFrom<String> for TextInput {
    type Error = SantaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
