use std::sync::Arc;

use nutype::nutype;

#[nutype(derive(Clone, AsRef))]
pub struct ChallengeRawInput(std::sync::Arc<String>);

impl AsRef<str> for ChallengeRawInput {
    fn as_ref(&self) -> &str {
        <Self as AsRef<Arc<String>>>::as_ref(self).as_str()
    }
}
