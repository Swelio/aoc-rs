use crate::solver::ChallengeRawInput;

use super::identity::Identity;

pub struct ChallengeRequest {
    id: Identity,
    input: ChallengeRawInput,
}

impl ChallengeRequest {
    pub fn new(id: Identity, input: ChallengeRawInput) -> Self {
        Self { id, input }
    }

    pub fn id(&self) -> Identity {
        self.id
    }

    pub fn input(&self) -> &ChallengeRawInput {
        &self.input
    }
}
