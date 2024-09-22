use crate::solver::request::ChallengeRawInput;

use super::identity::Identity;

pub struct ChallengeRequest {
    id: Identity,
    input: ChallengeRawInput,
}

impl ChallengeRequest {
    pub fn id(&self) -> Identity {
        self.id
    }

    pub fn input(&self) -> &ChallengeRawInput {
        &self.input
    }
}
