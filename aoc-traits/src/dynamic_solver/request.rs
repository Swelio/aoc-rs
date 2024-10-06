use crate::solver::raw_input::ChallengeRawInput;

use super::identity::Identity;

pub struct ChallengeRequest {
    id: Identity,
    input: ChallengeRawInput,
}

impl ChallengeRequest {
    pub fn new(identity: Identity, input: ChallengeRawInput) -> Self {
        Self {
            id: identity,
            input,
        }
    }

    pub fn id(&self) -> Identity {
        self.id
    }

    pub fn input(&self) -> &ChallengeRawInput {
        &self.input
    }
}