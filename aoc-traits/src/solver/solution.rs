use std::marker::PhantomData;

use nutype::nutype;

pub struct ChallengeSolution<Identity> {
    challenge: PhantomData<Identity>,
    solution: Solution,
}

#[nutype()]
pub struct Solution(i64);
