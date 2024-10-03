pub mod dynamic_solver;
pub mod solver;

pub use aoc_macros::DynamicSolver;

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
