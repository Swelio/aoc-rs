use super::{identity::Identity, input::ChallengeInput};

pub struct Challenge {
    identity: Identity,
    input: ChallengeInput,
}

impl Challenge {
    pub fn new(identity: Identity, input: ChallengeInput) -> Self {
        Self { identity, input }
    }

    pub fn identity(&self) -> Identity {
        self.identity
    }

    pub fn input(&self) -> &ChallengeInput {
        &self.input
    }
}
