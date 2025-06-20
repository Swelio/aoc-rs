use crate::error::{SantaError, SantaResult};

#[derive(Debug, Clone, derive_more::AsRef)]
#[as_ref(str)]
pub struct TextInput(String);

impl TextInput {
    pub fn try_new(input: &str) -> SantaResult<Self> {
        if input.is_empty() {
            return Err(SantaError::EmptyInput);
        }

        Ok(Self(input.to_string()))
    }
}

impl TryFrom<String> for TextInput {
    type Error = SantaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(&value)
    }
}
