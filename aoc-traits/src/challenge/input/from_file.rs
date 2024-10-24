use std::{fs, path::Path, sync::Arc};

use crate::error::AocError;

use super::ChallengeInput;

impl TryFrom<&Path> for ChallengeInput {
    type Error = AocError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(path)?;
        Ok(Self::new(Arc::new(content)))
    }
}
