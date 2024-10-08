pub mod challenge;
pub mod error;

pub use aoc_macros::DynamicSolver;
pub use dynamic_solver::DynamicSolver;
pub use solver::Solver;

mod dynamic_solver;
mod solver;

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
