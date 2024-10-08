use std::sync::Arc;

use nutype::nutype;

#[nutype(derive(Debug, Clone, AsRef))]
pub struct ChallengeInput(Arc<String>);

impl From<String> for ChallengeInput {
    fn from(content: String) -> Self {
        Self::new(Arc::new(content))
    }
}

impl AsRef<str> for ChallengeInput {
    fn as_ref(&self) -> &str {
        <Self as AsRef<Arc<String>>>::as_ref(self).as_str()
    }
}
