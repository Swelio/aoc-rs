use crate::error::{SantaError, SantaResult};

pub trait Parser<I>: Sized {
    fn try_parse(input: I) -> SantaResult<Self>;
}

#[diagnostic::do_not_recommend]
impl<T, I> Parser<I> for T
where
    T: TryFrom<I>,
    anyhow::Error: From<T::Error>,
{
    fn try_parse(input: I) -> SantaResult<Self> {
        T::try_from(input).map_err(|err| SantaError::ParsingError(err.into()))
    }
}
