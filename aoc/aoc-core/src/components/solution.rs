use std::marker::PhantomData;

use crate::SantaResult;

#[derive(Debug, PartialEq, Eq, derive_more::AsRef, serde::Serialize)]
#[serde(transparent)]
pub struct Solution<P: ?Sized> {
    #[as_ref(str)]
    value: String,
    #[serde(skip)]
    part: PhantomData<P>,
}

impl<P: ?Sized> Solution<P> {
    pub fn try_new<I>(solution: I) -> SantaResult<Self>
    where
        I: ToString,
    {
        let solution = solution.to_string();

        if solution.is_empty() {
            return Err(crate::SantaError::EmptyInput);
        }

        Ok(Self {
            value: solution,
            part: PhantomData,
        })
    }
}
