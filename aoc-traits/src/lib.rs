pub mod challenge_result;
pub mod dynamic_solver;
pub mod error;
pub mod solver;

pub use aoc_macros::DynamicSolver;
pub use challenge_result::ChallengeResult;
pub use dynamic_solver::{ChallengeRequest, ChallengeSolution, Day, DynamicSolver, Part, Year};
pub use error::{AocError, AocResult};
pub use solver::{ChallengeRawInput, Solution, Solver};

pub mod years {
    use aoc_macros::generate_year_units;

    generate_year_units!();
}

pub mod days {
    use aoc_macros::generate_day_units;

    generate_day_units!();
}

pub mod parts {
    use aoc_macros::generate_part_units;

    generate_part_units!();
}
