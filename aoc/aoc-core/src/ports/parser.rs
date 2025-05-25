use crate::error::ParsingResult;

pub trait Parser<I>: Sized {
    fn try_parse(input: I) -> ParsingResult<Self>;
}
