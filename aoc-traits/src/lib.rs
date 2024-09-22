pub mod executor;
pub mod solver;

pub mod years {
    use macros::generate_year_units;

    generate_year_units!();
}

pub mod days {
    use macros::generate_day_units;

    generate_day_units!();
}

pub mod parts {
    use macros::generate_part_units;

    generate_part_units!();
}
