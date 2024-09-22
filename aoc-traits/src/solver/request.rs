use std::marker::PhantomData;

use nutype::nutype;

pub struct ChallengeRequest<Identity> {
    challenge: PhantomData<Identity>,
    input: ChallengeRawInput,
}

#[nutype(derive(Clone, AsRef))]
pub struct ChallengeRawInput(std::sync::Arc<String>);
