use std::collections::HashMap;

use aoc_traits::{AocError, AocResult, ChallengeRequest, ChallengeSolution, DynamicSolver, Year};

/// Solve every year thanks to its inner yearly solvers
#[derive(Default)]
pub struct UniversalSolver(HashMap<Year, Box<dyn DynamicSolver>>);

impl DynamicSolver for UniversalSolver {
    fn resolve(&self, request: ChallengeRequest) -> AocResult<ChallengeSolution> {
        let challenge_id = request.id();

        self.0
            .get(&challenge_id.year())
            .map(|year_solver| year_solver.resolve(request))
            .unwrap_or_else(|| Err(AocError::NotImplemented(challenge_id)))
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
