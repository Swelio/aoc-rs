use super::{
    identity::Identity,
    input::{ChallengeInput, InputName},
};

pub struct Challenge {
    identity: Identity,
    input_name: InputName,
    input: ChallengeInput,
}

impl Challenge {
    pub fn new(identity: Identity, input_name: InputName, input: ChallengeInput) -> Self {
        Self {
            identity,
            input_name,
            input,
        }
    }

    pub fn identity(&self) -> Identity {
        self.identity
    }

    pub fn input_name(&self) -> &InputName {
        &self.input_name
    }

    pub fn input(&self) -> &ChallengeInput {
        &self.input
    }
}
