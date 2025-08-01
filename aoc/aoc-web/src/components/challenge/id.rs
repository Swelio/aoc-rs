use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub struct ChallengeId(Uuid);

impl ChallengeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ChallengeId {
    fn default() -> Self {
        Self::new()
    }
}
