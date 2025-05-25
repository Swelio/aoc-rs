pub use challenge::Challenge;
pub use identity::Identity;
pub use parser::Parser;

use crate::archetypes::DaySolution;

#[cfg(feature = "test")]
pub mod tests;

mod challenge;
mod identity;
mod parser;

impl<S, T> Challenge<DaySolution<S>> for T
where
    T: Challenge<S> + Identity,
{
    fn solve(&self) -> crate::SantaResult<DaySolution<S>> {
        let solutions: S = self.solve()?;
        let identity = T::get_identity();

        Ok(DaySolution::new(identity, solutions))
    }
}
