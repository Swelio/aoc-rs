use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct ChallengeIdentity<Year, Day, Part>(
    PhantomData<Year>,
    PhantomData<Day>,
    PhantomData<Part>,
);
