use std::collections::HashMap;

use aoc_traits::{
    challenge::{Challenge, Solution, Year},
    error::AocError,
    DynamicSolver,
};

/// Solve every year thanks to its inner yearly solvers
#[derive(Default)]
pub struct UniversalSolver(HashMap<Year, Box<dyn DynamicSolver>>);

impl DynamicSolver for UniversalSolver {
    fn resolve(&self, challenge: Challenge) -> Solution {
        let identity = challenge.identity();
        let input_name = challenge.input_name().to_owned();

        self.0
            .get(&identity.year())
            .map(|year_solver| year_solver.resolve(challenge))
            .unwrap_or_else(|| {
                Solution::new(
                    identity,
                    input_name,
                    Err(AocError::NotImplemented(identity)),
                )
            })
    }
}

impl UniversalSolver {
    pub fn add_solver(&mut self, year: Year, solver: impl DynamicSolver + 'static) -> &mut Self {
        self.0.insert(year, Box::new(solver));
        self
    }

    pub fn remove_solver(&mut self, year: Year) -> &mut Self {
        self.0.remove(&year);
        self
    }
}
