use crate::challenge::{Challenge, Solution};

pub trait DynamicSolver {
    fn resolve(&self, challenge: Challenge) -> Solution;
}
