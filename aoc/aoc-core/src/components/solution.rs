use std::marker::PhantomData;

use crate::SantaResult;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Into)]
pub struct Solution<P: ?Sized> {
    #[into]
    value: String,
    part: PhantomData<P>,
}

impl<P: ?Sized> Solution<P> {
    pub fn try_new<I>(solution: I) -> SantaResult<Self>
    where
        I: ToString,
    {
        let solution = solution.to_string();

        if solution.is_empty() {
            return Err(crate::SantaError::EmptySolution);
        }

        Ok(Self {
            value: solution,
            part: PhantomData,
        })
    }
}
